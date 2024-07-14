use std::{
  borrow::Cow,
  collections::{HashMap, HashSet},
};

use napi::Error;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

pub type Id = i32;

pub type CowStr = Cow<'static, str>;

const MAX_COUNT: usize = 12;
struct RedisKey {}

impl RedisKey {
  fn helpers() -> CowStr {
    return Cow::Borrowed("helpers");
  }

  fn user_to_region_ids(id: Id) -> CowStr {
    return Cow::Owned(format!("user_to_region_ids:{}", id));
  }
}

#[derive(Clone, Copy, Debug, sqlx::FromRow)]
pub struct Vehicle {
  id: Id,
  #[sqlx(rename = "regionId")]
  region_id: Id,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct AssignedVehicle {
  id: Id,
  locked: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Helper {
  id: Id,
  is_online: bool,
  #[serde(default, skip_serializing)]
  region_ids: Vec<Id>,
  #[serde(default)]
  vehicles: Vec<AssignedVehicle>,
}

#[napi]
#[derive(Debug)]
pub struct DistributionService {
  redis_client: redis::Client,
  sql_pool: sqlx::MySqlPool,
}

#[napi]
impl DistributionService {
  #[napi(factory)]
  pub async fn setup(redis_uri: String, mysql_uri: String) -> Self {
    let redis_client = redis::Client::open(redis_uri).expect("valid redis url");
    let sql_pool = sqlx::mysql::MySqlPoolOptions::new()
      .max_connections(1)
      .connect(&mysql_uri)
      .await
      .expect("connect successfully");
    DistributionService {
      redis_client,
      sql_pool,
    }
  }

  async fn prepare_data(&self) -> (Vec<Vehicle>, Vec<Helper>) {
    let mut tx = self.sql_pool.begin().await.expect("begin");
    let vehicles = sqlx::query_as::<_, Vehicle>("select * from vehicle")
      .fetch_all(&mut *tx)
      .await
      .expect("select vehicle");
    let mut redis_conn = self
      .redis_client
      .get_multiplexed_async_connection()
      .await
      .expect("redis conn");
    let key = RedisKey::helpers();
    let raw_helper_map: Vec<String> = redis_conn
      .hgetall(key.as_ref())
      .await
      .expect("redis hgetall");
    let mut helpers: Vec<Helper> = raw_helper_map
      .chunks(2)
      .map(|item| {
        let helper: Helper = serde_json::from_str(&item[1]).expect("ok");
        helper
      })
      .collect();
    let mut pipeline = redis::pipe();
    for helper in &helpers {
      let key = RedisKey::user_to_region_ids(helper.id);
      pipeline.get(key.as_ref());
    }
    let raw_values = redis_conn
      .send_packed_commands(&pipeline, 0, helpers.len())
      .await
      .expect("ok");
    for (i, value) in raw_values.into_iter().enumerate() {
      if let redis::Value::Data(s) = value {
        let region_ids: Vec<Id> =
          serde_json::from_str(&String::from_utf8(s).expect("ok")).expect("ok");
        helpers[i].region_ids = region_ids;
      }
    }
    (vehicles, helpers)
  }

  fn assign_vehicles(vehicles: &mut Vec<Vehicle>, helpers: &mut Vec<Helper>) -> HashSet<Id> {
    let online_vehicle_ids: HashSet<Id> = HashSet::from_iter(vehicles.iter().map(|v| v.id));
    let mut region_id_to_vehicle_ids: HashMap<Id, HashSet<Id>> = HashMap::new();
    let mut changed_ids = HashSet::new();
    for vehicle in vehicles {
      if let Some(group) = region_id_to_vehicle_ids.get_mut(&vehicle.region_id) {
        group.insert(vehicle.id);
      } else {
        let mut new_group = HashSet::new();
        new_group.insert(vehicle.id);
        region_id_to_vehicle_ids.insert(vehicle.region_id, new_group);
      }
    }
    for helper in helpers.iter_mut() {
      if !helper.is_online {
        helper.vehicles.clear();
        changed_ids.insert(helper.id);
      } else {
        helper.vehicles = helper
          .vehicles
          .iter()
          .filter_map(|v| {
            if online_vehicle_ids.contains(&v.id) {
              Some(*v)
            } else {
              changed_ids.insert(helper.id);
              None
            }
          })
          .collect();
      }
    }
    let mut online_helpers: Vec<&mut Helper> = helpers.iter_mut().filter(|v| v.is_online).collect();
    let online_helper_to_region_ids: HashMap<Id, HashSet<Id>> = HashMap::from_iter(
      online_helpers
        .iter()
        .map(|v| return (v.id, HashSet::from_iter(v.region_ids.iter().map(|&v| v)))),
    );

    for (region_id, vehicle_ids) in region_id_to_vehicle_ids {
      let mut region_helpers: Vec<_> = online_helpers
        .iter_mut()
        .filter_map(|v| {
          if online_helper_to_region_ids
            .get(&v.id)
            .unwrap()
            .contains(&region_id)
          {
            Some(v)
          } else {
            None
          }
        })
        .collect();
      if region_helpers.is_empty() {
        continue;
      }
      region_helpers.sort_by(|a, b| a.vehicles.len().cmp(&b.vehicles.len()));
      for &vehicle_id in &vehicle_ids {
        if region_helpers
          .iter()
          .any(|v| v.vehicles.iter().any(|v| v.id == vehicle_id))
        {
          continue;
        }
        if region_helpers[0].vehicles.len() >= MAX_COUNT {
          break;
        }
        region_helpers[0].vehicles.push(AssignedVehicle {
          id: vehicle_id,
          locked: false,
        });
        changed_ids.insert(region_helpers[0].id);
        if let Some(i) = (1..region_helpers.len())
          .rev()
          .find(|&i| region_helpers[i].vehicles.len() < region_helpers[0].vehicles.len())
        {
          region_helpers.swap(0, i);
        }
      }
      let mut i = region_helpers.len() - 1;
      while i != 0
        && region_helpers[0].vehicles.len() < MAX_COUNT
        && region_helpers[i].vehicles.len() > region_helpers[0].vehicles.len() + 1
      {
        if let Some(vehicle_index) = region_helpers[i]
          .vehicles
          .iter()
          .position(|v| !v.locked && vehicle_ids.contains(&v.id))
        {
          let vehicle = region_helpers[i].vehicles.remove(vehicle_index);
          changed_ids.insert(region_helpers[i].id);
          if let Some(j) =
            (1..i).find(|&j| region_helpers[j].vehicles.len() > region_helpers[i].vehicles.len())
          {
            region_helpers.swap(i, j);
          }
          region_helpers[0].vehicles.push(vehicle);
          changed_ids.insert(region_helpers[0].id);
          if let Some(j) = (1..region_helpers.len())
            .rev()
            .find(|&j| region_helpers[j].vehicles.len() < region_helpers[0].vehicles.len())
          {
            region_helpers.swap(0, j);
          }
        } else {
          i -= 1;
        }
      }
    }
    changed_ids
  }

  async fn persist_state(&self, helpers: Vec<Helper>, changed_ids: HashSet<Id>) {
    if changed_ids.is_empty() {
      return;
    }
    let mut redis_conn = self
      .redis_client
      .get_multiplexed_async_connection()
      .await
      .expect("ok");
    let mut pipeline = redis::pipe();
    for helper in helpers {
      if !changed_ids.contains(&helper.id) {
        continue;
      }
      pipeline.hset(
        RedisKey::helpers().as_ref(),
        helper.id,
        serde_json::to_string(&helper).expect("ok"),
      );
    }
    redis_conn
      .send_packed_commands(&pipeline, 0, changed_ids.len())
      .await
      .expect("ok");
  }

  #[napi]
  pub async fn cron_assign(&self) {
    let (mut vehicles, mut helpers) = self.prepare_data().await;
    let changed_ids = DistributionService::assign_vehicles(&mut vehicles, &mut helpers);
    self.persist_state(helpers, changed_ids).await;
  }

  #[napi]
  pub async fn lock_vehicle(&self, helper_id: Id, vehicle_id: Id) -> Result<(), Error> {
    let (mut vehicles, mut helpers) = self.prepare_data().await;
    if let Some(v) = vehicles.iter().find(|v| v.id == vehicle_id) {
      if let Some(h) = helpers.iter_mut().find(|v| v.id == helper_id) {
        if !h.is_online
          || !h.region_ids.contains(&v.region_id)
          || (h.vehicles.len() >= MAX_COUNT && h.vehicles.iter().all(|v| v.locked))
        {
          return Err(Error::from_reason("not allowed"));
        } else {
          for helper_vehicle in h.vehicles.iter_mut() {
            if helper_vehicle.id == vehicle_id {
              if !helper_vehicle.locked {
                helper_vehicle.locked = true;
                self
                  .persist_state(helpers, HashSet::from([vehicle_id]))
                  .await;
              }
              return Ok(());
            }
          }
          if h.vehicles.iter().any(|v| v.id == vehicle_id) {
            return Ok(());
          }
          if h.vehicles.len() < MAX_COUNT {
            h.vehicles.push(AssignedVehicle {
              id: vehicle_id,
              locked: true,
            })
          } else {
            let index = h.vehicles.iter().position(|v| !v.locked).unwrap();
            h.vehicles[index] = AssignedVehicle {
              id: vehicle_id,
              locked: true,
            }
          }
        }
      }

      let mut ids = Vec::with_capacity(2);
      ids.push(helper_id);
      for h in helpers.iter_mut() {
        if h.id == helper_id {
          continue;
        }
        if let Some(index) = h.vehicles.iter().position(|v| v.id == vehicle_id) {
          h.vehicles.remove(index);
          ids.push(h.id);
          break;
        }
      }
      let mut changed_ids = DistributionService::assign_vehicles(&mut vehicles, &mut helpers);
      changed_ids.extend(ids.into_iter());
      self.persist_state(helpers, changed_ids).await;
      return Ok(());
    }
    Err(Error::from_reason("not allowed"))
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashSet;

  use super::{AssignedVehicle, DistributionService, Helper, Vehicle};

  fn gen_test_data_1() -> (Vec<Vehicle>, Vec<Helper>) {
    let vehicles = vec![
      Vehicle {
        id: 1,
        region_id: 1,
      },
      Vehicle {
        id: 2,
        region_id: 2,
      },
    ];

    let helpers = vec![
      Helper {
        id: 1,
        is_online: true,
        region_ids: vec![1, 2],
        vehicles: vec![],
      },
      Helper {
        id: 2,
        is_online: false,
        region_ids: vec![1, 2],
        vehicles: vec![AssignedVehicle {
          id: 3,
          locked: true,
        }],
      },
    ];

    (vehicles, helpers)
  }

  #[test]
  fn test_basic() {
    let (mut vehicles, mut helpers) = gen_test_data_1();

    DistributionService::assign_vehicles(&mut vehicles, &mut helpers);
  }

  #[tokio::test]
  async fn test_setup() {
    let service = DistributionService::setup(
      "redis://:password@127.0.0.1:6379/0".to_string(),
      "mysql://user:password@127.0.0.1:3306/grpc".to_string(),
    )
    .await;
    let (_, helpers) = gen_test_data_1();
    service
      .persist_state(helpers, HashSet::from_iter(1..2))
      .await;
    service.cron_assign().await;
  }

  #[tokio::test]
  async fn test_lock_vehicle() {
    let service = DistributionService::setup(
      "redis://:password@127.0.0.1:6379/0".to_string(),
      "mysql://user:password@127.0.0.1:3306/grpc".to_string(),
    )
    .await;
    service.lock_vehicle(1, 1).await.expect_err("not allowed");
  }
}

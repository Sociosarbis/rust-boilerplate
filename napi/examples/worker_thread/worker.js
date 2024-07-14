const { parentPort, workerData } = require('node:worker_threads')
const { DistributionService } = require('../../index')

const servicePromise = DistributionService.setup(workerData.redis_uri, workerData.mysql_uri);

let id = 0;
parentPort?.on('message', (m) => {
  servicePromise.then((s) => {
    const _id = ++id;
    if (m.type === 'assign') {
      console.time(`cronAssign ${_id}`);
      s.cronAssign().then(() => {
        console.timeEnd(`cronAssign ${_id}`);
        parentPort.postMessage(`finished ${_id}`);
      });
    } else {
      console.time(`lockVehicle ${_id}`);
      s.lockVehicle(2, 1).then(() => {
        console.timeEnd(`lockVehicle ${_id}`);
        parentPort.postMessage(`finished ${_id}`);
      }).catch((e) => {
        console.warn(e.message);
      });
    }
  })
})
const { Worker } = require('node:worker_threads')
const { join } = require('node:path')

function spawn() {
  const worker = new Worker(join(__dirname, 'worker.js'), {
    workerData: {
      redis_uri: "redis://:password@127.0.0.1:6379/0",
      mysql_uri: "mysql://user:password@127.0.0.1:3306/grpc"
    }
  })

  worker.on('message', (data) => {
    console.log('data:%s', data)
  })

  worker.on('exit', () => {
    console.log('exit')
  })

  let i = 0;
  setInterval(() => {
    i++;
    worker.postMessage({ type: i % 2 == 0 ? 'assign' : 'lock' })
  }, 1e3)
}

spawn()
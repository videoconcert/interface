import("./pkg").then(module => {
  //const worker = new Worker('worker.bundle.js');
  //console.log('module', module);
  module.run_app();
});
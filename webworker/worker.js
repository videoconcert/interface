import("./pkg").then(module => {
  console.log('worker mod', module);
  module.run_worker();
})
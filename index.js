import init, { greet, rotate_shape, translate_shape } from "./pkg/hello_wasm.js";

init().then(() => {
  function rotate_square(length, orientation_in_degrees) {
    console.log(`rotate_square(${length}, ${orientation_in_degrees})`);
    let svg = rotate_shape(length, orientation_in_degrees);
    let base_div = document.getElementById('base_div');
    base_div.innerHTML = svg;
  }

  function translate_square(length, x) {
    console.log(`translate_square(${length}, ${x})`);
    let svg = translate_shape(length, x, 0);
    let base_div = document.getElementById('base_div');
    base_div.innerHTML = svg;
  }

  function go(starting_orientation, max_duration, step) {
    function schedule(f, duration) {
      if (duration <= 0) {
        return;
      }
      let next = f();
      setTimeout(schedule, step, next, duration - step);
    }

    let recurse = (f) => {
      let a = f();
      return () => recurse(a);
    }

    let f = (o) => {
      rotate_square(100, o);
      return () => f(o+=5);
    };

    let iterate = recurse(f(starting_orientation))
    
    schedule(iterate, max_duration);
  }

  window.go = go;

//  document.getElementById("tagline").addEventListener('click', function() {
//   go(100, 10000, 100);
//  }, false);

});

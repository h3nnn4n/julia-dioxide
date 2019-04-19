export const load = () => {
  import( /* webpackChunkName: "julia_dioxide" */ './pkg/julia_dioxide.js')
    .then(wasm => {
      wasm.init();
      const realInput = <HTMLInputElement>document.getElementById('real');
      const imaginaryInput = <HTMLInputElement>document.getElementById('imaginary');
      const renderBtn = document.getElementById('render');

      if (renderBtn == null || realInput == null || imaginaryInput == null) {
        console.log('Controls not found');
        return;
      }

      realInput.addEventListener('change', () => {
        wasm.render_julia_set();
      });

      imaginaryInput.addEventListener('change', () => {
        wasm.render_julia_set();
      });

      renderBtn.addEventListener('click', () => {
        wasm.render_julia_set();
      });
    })
    .catch(console.error);
}

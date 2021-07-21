import { Controller } from "stimulus"

export default class extends Controller {
  static targets = ["submit"];

  submit(event) {
    event.preventDefault()
    let form = this.element;
    const request = new Request('/bump-count');
    fetch(request)
      .then(response => response.json())
      .then(data => {
        console.log(data);
      });
  }
}
import { Controller } from "stimulus"

export default class extends Controller {
  static targets = ["submit", "output", "error"];

  submit(event) {
    event.preventDefault()
    if (this.submitTarget) {
      this.submitTarget.disabled = true;
    }
    const maybeOptimisticUpdateType = this.outputTarget?.dataset?.optimisticUpdateType;
    if (maybeOptimisticUpdateType) {
      this.optimistic(event, maybeOptimisticUpdateType);
    }
    const request = new Request('/bump-count');
    fetch(request)
      .then(response => response.json())
      .then(data => {
        console.log(data);
        if (!data.success && maybeOptimisticUpdateType) {
          this.revert(event, maybeOptimisticUpdateType);
        }
        console.log(this.errorTarget);
        if (!data.success && this.errorTarget) {
          this.errorTarget.classList.remove('hide');
        } else if (data.success && this.errorTarget) {
          this.errorTarget.classList.add('hide');
        }
      })
      .finally(() => {
        this.submitTarget.disabled = false;
      });
  }

  optimistic(event, type) {
    switch (type) {
      case 'increment': {
        const num = Number(this.outputTarget.innerHTML);
        if (Number.isNaN(num)) {
          throw new Error('Unable to increment non-number field');
        }
        this.outputTarget.innerHTML = num + 1;
        break;
      }
      default: {
        throw new Error("Unhandled optimistic update type ", type);
      }
    }
  }

  revert(event, type) {
    switch (type) {
      case 'increment': {
        const num = Number(this.outputTarget.innerHTML);
        if (Number.isNaN(num)) {
          throw new Error('Unable to revert non-number field');
        }
        this.outputTarget.innerHTML = num - 1;
        break;
      }
      default: {
        throw new Error("Unhandled revert type ", type);
      }
    }
  }
}
import { Controller } from "stimulus"

const toBase64 = file => new Promise((resolve, reject) => {
  const reader = new FileReader();
  reader.readAsDataURL(file);
  reader.onload = () => resolve(reader.result);
  reader.onerror = error => reject(error);
})

const isFile = (input) => !!input.type && !!input.lastModified

export default class extends Controller {
  static targets = ["submit", "output", "error"];

  async submit(event) {
    event.preventDefault()
    if (this.submitTarget) {
      this.submitTarget.disabled = true;
    }
    if (this.hasOutputTarget && this.outputTarget.dataset?.optimisticUpdateType) {
      this.optimistic(event, this.outputTarget.dataset.optimisticUpdateType);
    }
    const form = new FormData(event.target);
    const values = Object.fromEntries(form.entries());
    const body = {};
    for (const key in values) {
      if (isFile(values[key])) {
        body[key] = await toBase64(values[key])
      } else {
        body[key] = values[key]
      }
    }
    const requestOptions = {
      method: this.element.method
    };
    const params = new URLSearchParams(body);
    let url = this.element.action;
    if (this.element.method.toLowerCase() === 'get' && params.toString()) {
      url = `${this.element.action}?${params.toString()}`;
    } else if (this.element.method.toLowerCase() === 'post') {
      requestOptions.body = JSON.stringify(body)
    } else {
      console.error('unhandled form action method');
    }
    const request = new Request(url, requestOptions)
    fetch(request)
      .then(response => response.json())
      .then(data => {
        if (!data.success && maybeOptimisticUpdateType) {
          this.revert(event, maybeOptimisticUpdateType);
        }
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

  checkapi(event) {
    if (!event.target.value) {
      return;
    }
    const validationApiRoute = event.target?.dataset?.validationApiRoute;
    if (!validationApiRoute) {
      throw new Error('Missing API validation route!');
    }
    const validationResultElementId = event.target?.dataset?.validationResultElementId;
    if (!validationResultElementId) {
      throw new Error('Missing API validation result element ID!');
    }
    const validationResultElement = document.getElementById(validationResultElementId);
    if (!validationResultElement) {
      throw new Error('Missing API validation result element!');
    }
    const validationSuccessText = event.target?.dataset?.validationSuccessText;
    const validationFailureText = event.target?.dataset?.validationFailureText;
    if (!validationSuccessText || !validationFailureText) {
      throw new Error('Missing API validation result content!');
    }
    const maybeDivider = validationApiRoute[validationApiRoute.length - 1] === '/' ? '' : '/';
    const validationApiRouteFull = `${validationApiRoute}${maybeDivider}${event.target.value}`;
    const request = new Request(validationApiRouteFull);
    fetch(request)
      .then(response => response.json())
      .then(({ isValid }) => {
        validationResultElement.classList.remove('hide');
        validationResultElement.classList.remove('failure');
        validationResultElement.classList.remove('success');
        if (isValid) {
          validationResultElement.classList.add('success');
          validationResultElement.innerHTML = validationSuccessText;
        } else {
          validationResultElement.classList.add('failure');
          validationResultElement.innerHTML = validationFailureText;
        }
      });
  }
}
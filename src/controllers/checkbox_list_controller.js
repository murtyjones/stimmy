import { Controller } from "stimulus";

export default class extends Controller {
    static targets = ["count"];

    connect() {
        this.renderCount();
    }

    renderCount() {
        if (this.hasCountTarget) {
            const count = this.selectedCheckboxes.length;
            this.countTarget.innerHTML = `${count} selected`;
        }
    }

    checkAll() {
        this.setAllCheckboxes(true);
        this.renderCount();
    }

    checkNone() {
      this.setAllCheckboxes(false);
      this.renderCount();
    }

    onChecked() {
        this.renderCount();
    }

    setAllCheckboxes(checked) {
        this.checkboxes.forEach((el) => {
            const checkbox = el;

            if (!checkbox.disabled) {
                checkbox.checked = checked;
            }
        });
    }

    get selectedCheckboxes() {
        return this.checkboxes.filter((c) => c.checked);
    }

    get checkboxes() {
        return new Array(...this.element.querySelectorAll("input[type=checkbox]"));
    }
};

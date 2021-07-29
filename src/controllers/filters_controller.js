import { clearCache, visit } from "@hotwired/turbo";
import { Controller } from "stimulus";

export default class extends Controller {
    static targets = ["filter"];

    filter() {
        const url = `${window.location.pathname}?${this.params}`;
        clearCache();
        visit(url);
    }

    get params () {
        return this.filterTargets.map((t) => `${t.name}=${t.value}`).join('&');
    }
}
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
        const searchParams = new URLSearchParams();
        this.filterTargets.forEach(t => {
            if (t.type === 'checkbox' && !t.checked) {
                return;
            }
            searchParams.append(t.name, t.value);
        });
        return searchParams.toString();
    }
}
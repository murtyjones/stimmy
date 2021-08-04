# Pros of Stimulus.js + Turbo

- **High levels of code reusability** – Controllers do not have app–specific logic in them. E.g. if you have a controller for opening a modal, that controller can be reused to open different modals without writing any JavaScript
- **Most business logic lives on server** – Because your Stimulus controllers are concerned with low level view things (opening/closing a modal, showing/hiding a tab), the business logic can be contained to your server where it's 1. more reusable, and 2. easier to test without concern over the DOM

# Cons of Stimulus.js

- **Learning curve** – It's a totally different model of application development, and takes extra thought up front to figure out how to structure things
- **High data transfer cost** – Since you're sending actual HTML over the wire, this translates to (potentially a lot) more data than a single page app
- **Hard to debug mistakes in HTML files** – Because Stimulus relies on data attributes in template HTML files, it's hard to debug typos and missing data attributes (compared to JSX inline in a React app, for example).

# Questions

– Is it possible to build a progressive web application?
- How best do you document the usage of a controller in an HTML file? A [Storybook](https://storybook.js.org/) equivalent is needed, which could probably be home-grown
- How hard is it to achieve/maintain a level of latency in production (100-300ms) that makes Stimulus apps feel like SPAs?
- For one-off rich experiences too complex for Stimulus to make sense, how would one integrate something like React (or another SPA)?
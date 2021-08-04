Observation: Single Page Apps (SPAs) suck for app development in the long term
Hypothesis: The inevitable coupling of view logic, business logic, and state logic causes a mess of code that periodically needs rewriting
Test: Can a web app that's mostly server-side rendered be a better dev experience while still allowing for a modern user experience?

[See the findings](./PROS_AND_CONS.md)

# To Run
- git clone `https://github.com/murtyjones/stimmy.git`
- cd `stimmy`
- `npm i`
- `rustup override set nightly`
- `npm start`
- go to http://localhost:8000
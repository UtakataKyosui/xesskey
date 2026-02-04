import { Workflow } from "ghats/dist/lib/package/workflow.js";
import { Job } from "ghats/dist/lib/package/job.js";

function action(name: string, params: any = {}) {
    return { name, params };
}

const ci = new Workflow("CI", {
    on: {
        push: { branches: ["main"] },
        pull_request: {},
    },
});

const testJob = new Job("Test", {
    "runs-on": "ubuntu-latest",
})
    .uses(action("actions/checkout"))

    .run("Rust Checks (xq-sns-app)", "cd xq-sns-app && cargo fmt --all -- --check && cargo clippy -- -D warnings && cargo test")
    .run("Rust Checks (PLocoBuf)", "cd PLocoBuf && cargo test")
    .run("Frontend Install", "cd xq-sns-app/frontend && npm install")
    .run("Frontend Lint & Build", "cd xq-sns-app/frontend && npm run lint && npm run build");

ci.addJob(testJob);

export default ci;

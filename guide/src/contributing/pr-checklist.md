# PR Checklist

When creating a PR, there are a few things you can do to help the process of
landing your changes go smoothly:

- **Code Formatting** - The most common step to forget is running `cargo fmt
  --all` (I forget all the time)! This step ensures there is a consistent style
  across splatter's codebase.
- **Errors** - All changes must build successfully, and all tests must complete
  successfully before we can merge each PR. Keep in mind that running tests
  locally can take a loooooong time, so sometimes it can be easier to open your
  PR and let the repo bot check for you.
- **Warnings** - Make sure to address any lingering code warnings before your
  last commit. Keep in mind that sometimes warnings already exist due to changes
  in the compiler's linter between versions. Try to at least make sure that your
  changes do not add any new ones :)
- **Check Examples** - Make sure the examples still work by running 
`cargo run --bin run_all_examples`. This executes a script that builds and runs
all the examples in the project. You need to do a manual check to see
if examples have failed to run successfully (e.g. check for wgpu validation errors).
- **Documentation** - If you have made any changes that could benefit from
  updating some code documentation, please be sure to do so! Try to put yourself
  in the shoes of someone reading your code for the first time.
- **Changelog** - [The changelog][splatter-changelog] acts as a human-friendly
  history of the repo that becomes especially useful to community members when
  updating between different versions of splatter. Be sure to add your changes to
  `guide/src/changelog.md`.
- **PR Comment** - Be sure to add the following to your PR to make it easier for
  reviewers to understand and land your code:
    - **Motivation** for changes. What inspired the PR? Please link to any
      related issues.
    - **Summary** of changes. Anything that might help the reviewer to
      understand your what your changes do will go a long way!

If you forget one of these steps before making your PR, don't panic! The splatter
repo has a CI (continuous integration) bot that will check for some of these
steps and notify you if anything is out of order. Once the bot checks pass, a
community member will review the rest.

[splatter-changelog]: https://guide.splatter.cc/changelog.html

# TODOS

## Bugs

- Overly long lines bleed off the page.
  - [ ] Include ellipses for overly long lines of code
- Current editing exp. makes it difficult to add long lines of text
  - [ ] Edits to trigger a fullscreen overlaid textbox

## Primary 

- (CRITICAL) Cannot change order of execution
  - [ ] Implement a drag and drop feature
- (CRITICAL) We cannot currently tell the result of a git pull
    - [ ] `run()` returns an array of strings. In svelte, we have a scrolling console that logs the `name` and `path` of every `spawn`, and the full output for every `output`
- (CRITICAL) Cannot currently have mutliline commands
  - There's a couple of ways to approach this. One is to allow multine commands, another is to allow grouping (so multiple commands can be run together)
  
-  [x] Toggle elements on and off based off of click on full box (not just checkbox)

## Features

- No icons
  - Have svelte `Images` make a call to `image` func ()
- No syntax higlighting
  - [ ] Use `prism.js` to highlight the shell arguments

# 10.21.2021

## Back

- Declare `Action` struct
  - name: String
  - icon: Icon
    - path: String
  - action_type: enum ActionType
    - Open
      - path to program: String
      - args: String
    - GitPull
      -  git-dir: String

### Serialization

- Use serde_json for saving user config
- define `save_config()` function that gets called

## Front
 
- Also declare the same `Action` struct 
  - (any way to import rust struct into typescript?)
  
```svelte
export let actions: Action[] = [];
let edit: bool = false;

{#if edit}
  {#each actions as action, i}
    {name}
    <image href={image.load(icon)}/>
    {action_type}
  {/each}
{:else}
  {#each actions as action, i}
    <div contenteditable=true>{name}</div>
    <image href={image.load(icon)}/>
    <div contenteditatble=true>{action_type}</div>
  {/each}
{/if edit}
```


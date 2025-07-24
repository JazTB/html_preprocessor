# html_preprocessor
A static preprocessor for HTML/CSS.

This tool (really don't use this unless you're curious) allows you to add preprocessor directives to HTML (inside `<!--@` and `@-->`) and CSS (`/*@` and `@*/`).
These directives are simple commands which accept simple (string) arguments, and replace themselves with some text.

A directive might look like:

```html
<div>
  <p> Some stuff here... </p>
  <!--@ div_of_class foo_class @-->
</div>
```

and turn into:

```html
<div>
  <p> Some stuff here... </p>
  <div class="foo_class">
  </div>
</div>
```

These directives are baked into the source code.
There's a big match statement ([Here for HTML](https://github.com/JazTB/html_preprocessor/blob/716e255/src/main.rs#L150) [Here for CSS](https://github.com/JazTB/html_preprocessor/blob/716e255/src/main.rs#L269)) where you can manually add your own directives if you add a new pattern, and manually parse arguments out of the `args` vec.

## Usage

This tool is run on a directory.

The directory is expected to have a `pp.json`, which is expected to look something like:

```json
{  
  "title": "My website!", // can be inserted with <@!-- title @-->
  "out_dir": "out", // directory will be created relative to your project dir
  "pp_files": [
    { "input": "index.html.pp", "output": "index.html" } // Directories aren't created automatically (WILL CHANGE)
  ],
  "copy_files": [
    "favicon.ico" // these files just get copied straight to the root. Directories aren't created automatically (WILL CHANGE)
  ]
}
```

Files in `pp_files` will be preprocessed. Files in `copy_files` will not.
Files which are included with the `include_file` directive (or the `css` directive with css) will be (recursively) preprocessed.

## /Users/user/project/out.js
### esbuild
```js
// Users/user/project/src/demo-pkg/no-ext-browser/index.js
var browser = "browser";

// Users/user/project/src/demo-pkg/no-ext/index.js
var node = "node";

// Users/user/project/src/demo-pkg/ext-browser/index.js
var browser2 = "browser";

// Users/user/project/src/entry.js
console.log(browser);
console.log(node);
console.log(browser2);
console.log(browser2);
```
### rolldown
```js

```
### diff
```diff
===================================================================
--- esbuild	/Users/user/project/out.js
+++ rolldown	
@@ -1,7 +0,0 @@
-var browser = "browser";
-var node = "node";
-var browser2 = "browser";
-console.log(browser);
-console.log(node);
-console.log(browser2);
-console.log(browser2);

```

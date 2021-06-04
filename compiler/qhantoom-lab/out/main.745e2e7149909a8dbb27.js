/******/ (function(modules) { // webpackBootstrap
/******/ 	// install a JSONP callback for chunk loading
/******/ 	function webpackJsonpCallback(data) {
/******/ 		var chunkIds = data[0];
/******/ 		var moreModules = data[1];
/******/
/******/
/******/ 		// add "moreModules" to the modules object,
/******/ 		// then flag all "chunkIds" as loaded and fire callback
/******/ 		var moduleId, chunkId, i = 0, resolves = [];
/******/ 		for(;i < chunkIds.length; i++) {
/******/ 			chunkId = chunkIds[i];
/******/ 			if(Object.prototype.hasOwnProperty.call(installedChunks, chunkId) && installedChunks[chunkId]) {
/******/ 				resolves.push(installedChunks[chunkId][0]);
/******/ 			}
/******/ 			installedChunks[chunkId] = 0;
/******/ 		}
/******/ 		for(moduleId in moreModules) {
/******/ 			if(Object.prototype.hasOwnProperty.call(moreModules, moduleId)) {
/******/ 				modules[moduleId] = moreModules[moduleId];
/******/ 			}
/******/ 		}
/******/ 		if(parentJsonpFunction) parentJsonpFunction(data);
/******/
/******/ 		while(resolves.length) {
/******/ 			resolves.shift()();
/******/ 		}
/******/
/******/ 	};
/******/
/******/
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// object to store loaded and loading chunks
/******/ 	// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 	// Promise = chunk loading, 0 = chunk loaded
/******/ 	var installedChunks = {
/******/ 		"main": 0
/******/ 	};
/******/
/******/
/******/
/******/ 	// script path function
/******/ 	function jsonpScriptSrc(chunkId) {
/******/ 		return __webpack_require__.p + "" + ({}[chunkId]||chunkId) + "." + "745e2e7149909a8dbb27" + ".js"
/******/ 	}
/******/
/******/ 	// object to store loaded and loading wasm modules
/******/ 	var installedWasmModules = {};
/******/
/******/ 	function promiseResolve() { return Promise.resolve(); }
/******/
/******/ 	var wasmImportObjects = {
/******/ 		"./src/runtime/pkg/qhantoom_lab_bg.wasm": function() {
/******/ 			return {
/******/
/******/ 			};
/******/ 		},
/******/ 	};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/ 	// This file contains only the entry chunk.
/******/ 	// The chunk loading function for additional chunks
/******/ 	__webpack_require__.e = function requireEnsure(chunkId) {
/******/ 		var promises = [];
/******/
/******/
/******/ 		// JSONP chunk loading for javascript
/******/
/******/ 		var installedChunkData = installedChunks[chunkId];
/******/ 		if(installedChunkData !== 0) { // 0 means "already installed".
/******/
/******/ 			// a Promise means "currently loading".
/******/ 			if(installedChunkData) {
/******/ 				promises.push(installedChunkData[2]);
/******/ 			} else {
/******/ 				// setup Promise in chunk cache
/******/ 				var promise = new Promise(function(resolve, reject) {
/******/ 					installedChunkData = installedChunks[chunkId] = [resolve, reject];
/******/ 				});
/******/ 				promises.push(installedChunkData[2] = promise);
/******/
/******/ 				// start chunk loading
/******/ 				var script = document.createElement('script');
/******/ 				var onScriptComplete;
/******/
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.src = jsonpScriptSrc(chunkId);
/******/
/******/ 				// create error before stack unwound to get useful stacktrace later
/******/ 				var error = new Error();
/******/ 				onScriptComplete = function (event) {
/******/ 					// avoid mem leaks in IE.
/******/ 					script.onerror = script.onload = null;
/******/ 					clearTimeout(timeout);
/******/ 					var chunk = installedChunks[chunkId];
/******/ 					if(chunk !== 0) {
/******/ 						if(chunk) {
/******/ 							var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 							var realSrc = event && event.target && event.target.src;
/******/ 							error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 							error.name = 'ChunkLoadError';
/******/ 							error.type = errorType;
/******/ 							error.request = realSrc;
/******/ 							chunk[1](error);
/******/ 						}
/******/ 						installedChunks[chunkId] = undefined;
/******/ 					}
/******/ 				};
/******/ 				var timeout = setTimeout(function(){
/******/ 					onScriptComplete({ type: 'timeout', target: script });
/******/ 				}, 120000);
/******/ 				script.onerror = script.onload = onScriptComplete;
/******/ 				document.head.appendChild(script);
/******/ 			}
/******/ 		}
/******/
/******/ 		// Fetch + compile chunk loading for webassembly
/******/
/******/ 		var wasmModules = {"1":["./src/runtime/pkg/qhantoom_lab_bg.wasm"]}[chunkId] || [];
/******/
/******/ 		wasmModules.forEach(function(wasmModuleId) {
/******/ 			var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/
/******/ 			// a Promise means "currently loading" or "already loaded".
/******/ 			if(installedWasmModuleData)
/******/ 				promises.push(installedWasmModuleData);
/******/ 			else {
/******/ 				var importObject = wasmImportObjects[wasmModuleId]();
/******/ 				var req = fetch(__webpack_require__.p + "" + {"./src/runtime/pkg/qhantoom_lab_bg.wasm":"53a7488ed0136fb586a0"}[wasmModuleId] + ".module.wasm");
/******/ 				var promise;
/******/ 				if(importObject instanceof Promise && typeof WebAssembly.compileStreaming === 'function') {
/******/ 					promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 						return WebAssembly.instantiate(items[0], items[1]);
/******/ 					});
/******/ 				} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 					promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 				} else {
/******/ 					var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 					promise = bytesPromise.then(function(bytes) {
/******/ 						return WebAssembly.instantiate(bytes, importObject);
/******/ 					});
/******/ 				}
/******/ 				promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 					return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 				}));
/******/ 			}
/******/ 		});
/******/ 		return Promise.all(promises);
/******/ 	};
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/ 	// on error function for async loading
/******/ 	__webpack_require__.oe = function(err) { console.error(err); throw err; };
/******/
/******/ 	// object with all WebAssembly.instance exports
/******/ 	__webpack_require__.w = {};
/******/
/******/ 	var jsonpArray = window["webpackJsonp"] = window["webpackJsonp"] || [];
/******/ 	var oldJsonpFunction = jsonpArray.push.bind(jsonpArray);
/******/ 	jsonpArray.push = webpackJsonpCallback;
/******/ 	jsonpArray = jsonpArray.slice();
/******/ 	for(var i = 0; i < jsonpArray.length; i++) webpackJsonpCallback(jsonpArray[i]);
/******/ 	var parentJsonpFunction = oldJsonpFunction;
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./src/runtime/index.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./src/runtime/box.js":
/*!****************************!*\
  !*** ./src/runtime/box.js ***!
  \****************************/
/*! exports provided: Box */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"Box\", function() { return Box; });\nconst Box = x => ({\n  map: f => Box(f(x)),\n  fold: f => f(x),\n  text: _ => `Box(${ x })`,\n});\n\n\n//# sourceURL=webpack:///./src/runtime/box.js?");

/***/ }),

/***/ "./src/runtime/const.js":
/*!******************************!*\
  !*** ./src/runtime/const.js ***!
  \******************************/
/*! exports provided: SAMPLE, DEFAULT_EDITOR_OPTION */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"SAMPLE\", function() { return SAMPLE; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"DEFAULT_EDITOR_OPTION\", function() { return DEFAULT_EDITOR_OPTION; });\nconst SAMPLE = Object.freeze({\n  empty: '',\n  simple: 'fun main := () {\\n\\t3 + 39;\\n}',\n  function: 'fun main := () {\\n\\tsqrt(9);\\n}\\n\\nfun srqt: (s8) -> s8 = (x) {\\n\\tx * x\\n}',\n  if: 'fun main := () {\\n\\tif true { 1 } else { 0 }\\n}',\n  array: 'fun main := () {\\n\\tval a : [] = [1, 2, 3];\\n\\ta[2];\\n}',\n  hash: 'fun main := () {\\n\\tval a : hash = { \"firstname\": \"john\", \"lastname\": \"doe\" };\\n\\ta[\"firstname\"];\\n}',\n  unary: 'fun main := () {\\n\\tval a : bool = !true;\\n\\tval b : bool = !0;\\n\\ta == b;\\n}',\n  int: 'fun main := () {\\n\\tval a : int = 2;\\n\\tval b : int = 2;\\n\\tval c : int = a + b;\\n\\tc;\\n}',\n  bigint: 'fun main := () {\\n\\tval a : int = 1_000_000_000;\\n\\tval b : int = 1_000_000;\\n\\tval c : int = a + b;\\n\\tc;\\n}',\n  float: 'fun main := () {\\n\\tval a : int = 1.234;\\n\\tval b : int = 1e4;\\n\\tval c : int = a + b;\\n\\tc;\\n}',\n  str: 'fun main := () {\\n\\tval a : str = \"abc\";\\n\\tval b : str = \"def\";\\n\\tval c : str = a + b;\\n\\tc;\\n}',\n  fibonacci: encodeURI(`\nfun main := {\n  fun fibonacci: Vec<uint> = (n) {\n    mut x1 := [1, 1];\n\n    for 2.=n = i {\n      val x2 := x1[i - 1] + x1[i - 2];\n      x1.push(x2);\n    }\n\n    x1\n  }\n\n  print(fibonacci(7));\n}\n`),\n  hello: encodeURI(`\n<script>\n  val name = \"world\";\n</script> \n\n<h1>hello, {name}!</h1>\n  `),\n});\n\nconst DEFAULT_EDITOR_OPTION = Object.freeze({\n  lineNumbers: true,\n  mode: \"qhantoom\",\n  theme: \"monokai\",\n  indentWithTabs: true,\n  tabSize: 2,\n  viewportMargin: Infinity,\n});\n\n\n//# sourceURL=webpack:///./src/runtime/const.js?");

/***/ }),

/***/ "./src/runtime/identity.js":
/*!*********************************!*\
  !*** ./src/runtime/identity.js ***!
  \*********************************/
/*! exports provided: identity */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"identity\", function() { return identity; });\nconst identity = x => x;\n\n\n//# sourceURL=webpack:///./src/runtime/identity.js?");

/***/ }),

/***/ "./src/runtime/index.js":
/*!******************************!*\
  !*** ./src/runtime/index.js ***!
  \******************************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _const__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./const */ \"./src/runtime/const.js\");\n/* harmony import */ var _box__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./box */ \"./src/runtime/box.js\");\n/* harmony import */ var _identity__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./identity */ \"./src/runtime/identity.js\");\n\n\n\n\n(async _ => {\n  const wasm = await Promise.all(/*! import() */[__webpack_require__.e(0), __webpack_require__.e(1)]).then(__webpack_require__.bind(null, /*! ./pkg/qhantoom_lab */ \"./src/runtime/pkg/qhantoom_lab.js\"))\n    .then(_identity__WEBPACK_IMPORTED_MODULE_2__[\"identity\"])\n    .catch(console.error);\n\n  const idToElmt = id => document.getElementById(id);\n\n  const compiler = ({tokenize, parse, analyze, interpret} = {}) => ({\n    tokenize: s => tokenize(s),\n    parse: s => parse(s),\n    analyze: s => analyze(s),\n    interpret: s => interpret(s),\n  });\n\n  const editor = _ => ({\n    view: {\n      input: makeEditor(\"editorInput\"),\n      output: document.getElementById(\"editorOutput\"),\n      // output: makeEditor(\"editorOutput\"),\n    },\n    select: {\n      sample: makeSelect(\"editorOutputDebug\", [\"tokens\", \"astree\", \"object\", \"iframe\"]),\n      // sample: makeSelect(\"commandControlsSample\", Object.keys(SAMPLE)),\n    },\n    cmd: {\n      clipboard: text => navigator.clipboard.writeText(text),\n    },\n  });\n\n  const makeEditor = id => {\n    const view = Object(_box__WEBPACK_IMPORTED_MODULE_1__[\"Box\"])(id)\n      .map(idToElmt)\n      .fold(_identity__WEBPACK_IMPORTED_MODULE_2__[\"identity\"]);\n  \n    const editor = Object(_box__WEBPACK_IMPORTED_MODULE_1__[\"Box\"])(view)\n      .map(viewToEditor)\n      .fold(_identity__WEBPACK_IMPORTED_MODULE_2__[\"identity\"]);\n  \n    return editor;\n  };\n  \n  const makeSelect = (id, options) => {\n    const select = Object(_box__WEBPACK_IMPORTED_MODULE_1__[\"Box\"])(id)\n      .map(idToElmt)\n      .fold(_identity__WEBPACK_IMPORTED_MODULE_2__[\"identity\"]);\n  \n    [...options].forEach((option) => {\n      Object(_box__WEBPACK_IMPORTED_MODULE_1__[\"Box\"])([\"option\", option])\n        .map(tagToElmt)\n        .fold(e => select.appendChild(e));\n    });\n  \n    return select;\n  };\n  \n  const tagToElmt = ([tagname, option]) => {\n    const elmt = document.createElement(tagname);\n    \n    elmt.innerHTML = _const__WEBPACK_IMPORTED_MODULE_0__[\"SAMPLE\"][option];\n    elmt.label = option;\n    elmt.value = option;\n  \n    return elmt;\n  };\n  \n  const viewToEditor = view => (\n    CodeMirror.fromTextArea(view, {\n      ..._const__WEBPACK_IMPORTED_MODULE_0__[\"DEFAULT_EDITOR_OPTION\"],\n    })\n  );\n\n  const output = _ => ({\n    debug: {\n      tokens: idToElmt(\"editorOutput\"),\n      astree: idToElmt(\"compilerOutputAstree\"),\n      object: idToElmt(\"compilerOutputObject\"),\n    },\n  });\n\n  const getCurrentOption = payload => {\n    const options = payload;\n    const selectedIndex = options.selectedIndex;\n  \n    return options[selectedIndex];\n  };\n\n  const app = {\n    component: {\n      compiler: Object(_box__WEBPACK_IMPORTED_MODULE_1__[\"Box\"])(compiler(wasm)),\n      editor: Object(_box__WEBPACK_IMPORTED_MODULE_1__[\"Box\"])(editor()),\n      output: Object(_box__WEBPACK_IMPORTED_MODULE_1__[\"Box\"])(output()),\n      iframe: Object(_box__WEBPACK_IMPORTED_MODULE_1__[\"Box\"])(document.createElement(\"iframe\")),\n    },\n    async start() {\n      const me = this;\n      const component = {...me.component};\n\n      Object.assign(me, {\n        component: Object\n          .keys(component)\n          .reduce((res, key) => ({\n            ...res,\n            [key]: res[key].fold(x => x),\n          }),\n          component,\n        ),\n      });\n\n      const {editor, compiler, output, iframe} = me.component;\n      const selectOptions = [...editor.select.sample.options];\n      const currentOptionName = \"function\";\n      const currentSample = _const__WEBPACK_IMPORTED_MODULE_0__[\"SAMPLE\"][currentOptionName].trim();\n\n      const currentKey = selectOptions.findIndex(\n        o => o.label === currentOptionName\n      );\n\n      iframe.classList.add(\"iframe-output\", \"box\", \"flex\")\n      iframe.src = \"data:text/html;charset=utf-8,\" + encodeURI(`\n<!DOCTYPE html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"UTF-8\">\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n    <title>Document</title>\n    <style>\n      *,\n      *:after,\n      *:before {\n        box-sizing: border-box;\n        -webkit-font-smoothing: antialiased;\n        -moz-osx-font-smoothing: grayscale;\n      }\n      html,\n      body {\n        width: 100vw;\n        height: 100vh;\n        overflow: hidden;\n        background: white;\n      }\n      body {\n        position: relative;\n      }\n    </style>\n  </head>\n  <body>\n      <h1>hello, world</h1>\n  </body>\n</html>\n      `);\n\n      // const iddi = document.getElementById(\"iddi\");\n      // iddi.appendChild(iframe);\n\n      editor.view.input.setValue(currentSample);\n      // editor.view.output.setValue(compiler.interpret(currentSample));\n\n      output.debug.tokens.innerHTML = compiler.tokenize(currentSample);\n      output.debug.astree.innerHTML = compiler.parse(currentSample);\n      output.debug.object.innerHTML = compiler.analyze(currentSample);\n      editor.select.sample.selectedIndex = currentKey;\n\n      editor.select.sample.addEventListener(\"change\", (event) => {\n        event.preventDefault();\n\n        const selectOptions = event.target.options;\n        const currentOption = getCurrentOption(selectOptions);\n        const currentSample = decodeURI(currentOption.textContent).trim();\n\n        editor.view.input.setValue(currentSample);\n\n        output.debug.tokens.innerHTML = compiler.tokenize(currentSample);\n        output.debug.astree.innerHTML = compiler.parse(currentSample);\n        output.debug.object.innerHTML = compiler.analyze(currentSample);\n      });\n    },\n    error: e => new Error(e),\n  };\n\n  try {\n    await app.start();\n  } catch(e) {\n    console.log(e);\n    throw app.error(e);\n  }\n})();\n\n\n//# sourceURL=webpack:///./src/runtime/index.js?");

/***/ })

/******/ });
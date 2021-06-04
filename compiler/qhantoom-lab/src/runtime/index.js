import {DEFAULT_EDITOR_OPTION, DEFAULT_IFRAME_VIEW, HOWTO} from "./const";
import {Box} from "./box";
import {identity} from "./identity";

(async _ => {
  const wasm = await import('./pkg/qhantoom_lab')
    .then(identity)
    .catch(console.error);

  const idToElmt = id => document.getElementById(id);

  const compiler = ({tokenize, parse, analyze, interpret} = {}) => ({
    tokenize: s => tokenize(s),
    parse: s => parse(s),
    analyze: s => analyze(s),
    interpret: s => interpret(s),
  });

  const editor = _ => ({
    view: {
      input: makeEditor("editorInput"),
      output: document.getElementById("editorOutput"),
    },
    select: {
      sample: makeSelect("editorOutputDebug", [
        "tokens", "astree", "object", "iframe", "output",
      ]),
    },
    cmd: {
      clipboard: text => navigator.clipboard.writeText(text),
    },
  });

  const makeEditor = id => {
    const view = Box(id)
      .map(idToElmt)
      .fold(identity);
  
    const editor = Box(view)
      .map(viewToEditor)
      .fold(identity);
  
    return editor;
  };
  
  const makeSelect = (id, options) => {
    const select = Box(id)
      .map(idToElmt)
      .fold(identity);
  
    [...options].forEach(option => {
      Box(["option", option])
        .map(tagToElmt)
        .fold(e => select.appendChild(e));
    });
  
    return select;
  };
  
  const tagToElmt = ([tagname, option]) => {
    const elmt = document.createElement(tagname);
    
    elmt.innerHTML = HOWTO[option];
    elmt.label = option;
    elmt.value = option;
  
    return elmt;
  };
  
  const viewToEditor = view => (
    CodeMirror.fromTextArea(view, {
      ...DEFAULT_EDITOR_OPTION,
    })
  );

  const output = _ => ({
    debug: {
      tokens: idToElmt("editorOutput"),
      astree: idToElmt("compilerOutputAstree"),
      object: idToElmt("compilerOutputObject"),
    },
  });

  const getCurrentOption = payload => {
    const options = payload;
    const selectedIndex = options.selectedIndex;
  
    return options[selectedIndex];
  };

  const app = {
    state: {
      current: {
        input: "",
        ouput: "",
      },
    },
    component: {
      compiler: Box(compiler(wasm)),
      editor: Box(editor()),
      output: Box(output()),
      iframe: Box(document.createElement("iframe")),
    },
    async start() {
      const me = this;
      const component = {...me.component};

      Object.assign(me, {
        component: Object
          .keys(component)
          .reduce((res, key) => ({
            ...res,
            [key]: res[key].fold(identity),
          }),
          component,
        ),
      });

      const {editor, compiler, output, iframe} = me.component;

      const onclick = event => {
        event.preventDefault();

        const target = event.target;
        const name = target.innerHTML;
        const sample = encodeURI(HOWTO[name]);
        const decoded = decodeURI(sample).trim();

        me.state.current.input = decoded;
        me.state.current.ouput = compiler.tokenize(decoded);
        
        editor.view.input.setValue(me.state.current.input);
        output.debug.tokens.innerHTML = me.state.current.ouput;
      };

      const howto = document.getElementById("howto");
      Object.keys(HOWTO).forEach(k => {
        const a = document.createElement("button");
        const e = document.createElement("li");

        a.classList.add("button", "button--howto");
        a.onclick = onclick;
        a.innerHTML = k;
        e.appendChild(a);
        howto.appendChild(e);
      });

      const currentOptionName = "start";
      const currentSample = decodeURI(encodeURI(HOWTO[currentOptionName])).trim();

      editor.view.input.setValue(currentSample);
      editor.view.output.innerHTML = compiler.tokenize(currentSample);
      me.state.current.input = currentSample;
      me.state.current.ouput = currentSample;

      editor.select.sample.addEventListener("change", event => {
        event.preventDefault();

        const selectOptions = event.target.options;
        const option = getCurrentOption(selectOptions);
        const decoded = decodeURI(me.state.current.input).trim();

        switch(option.label) {
          case "tokens":
            editor.view.output.innerHTML = compiler.tokenize(decoded);
            break;
          case "astree":
            editor.view.output.innerHTML = compiler.parse(decoded);
            break;
          case "object":
            editor.view.output.innerHTML = compiler.interpret(decoded);
            break;
          case "output":
            editor.view.output.innerHTML = compiler.interpret(decoded);
            break;
          case "iframe":
            editor.view.output.innerHTML = compiler.interpret(decoded);
            iframe.classList.add("iframe-output");
            iframe.src = "data:text/html;charset=utf-8," + encodeURI(DEFAULT_IFRAME_VIEW);
            break;
          default:
            return;
        }
      });
    },
    error: e => new Error(e),
  };

  try {
    await app.start();
  } catch(e) {
    console.log(e);
    throw app.error(e);
  }
})();

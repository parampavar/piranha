"use strict";(self.webpackChunkmy_website=self.webpackChunkmy_website||[]).push([[724],{5680:(e,r,t)=>{t.d(r,{xA:()=>p,yg:()=>y});var n=t(6540);function a(e,r,t){return r in e?Object.defineProperty(e,r,{value:t,enumerable:!0,configurable:!0,writable:!0}):e[r]=t,e}function s(e,r){var t=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);r&&(n=n.filter((function(r){return Object.getOwnPropertyDescriptor(e,r).enumerable}))),t.push.apply(t,n)}return t}function i(e){for(var r=1;r<arguments.length;r++){var t=null!=arguments[r]?arguments[r]:{};r%2?s(Object(t),!0).forEach((function(r){a(e,r,t[r])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(t)):s(Object(t)).forEach((function(r){Object.defineProperty(e,r,Object.getOwnPropertyDescriptor(t,r))}))}return e}function o(e,r){if(null==e)return{};var t,n,a=function(e,r){if(null==e)return{};var t,n,a={},s=Object.keys(e);for(n=0;n<s.length;n++)t=s[n],r.indexOf(t)>=0||(a[t]=e[t]);return a}(e,r);if(Object.getOwnPropertySymbols){var s=Object.getOwnPropertySymbols(e);for(n=0;n<s.length;n++)t=s[n],r.indexOf(t)>=0||Object.prototype.propertyIsEnumerable.call(e,t)&&(a[t]=e[t])}return a}var l=n.createContext({}),c=function(e){var r=n.useContext(l),t=r;return e&&(t="function"==typeof e?e(r):i(i({},r),e)),t},p=function(e){var r=c(e.components);return n.createElement(l.Provider,{value:r},e.children)},u="mdxType",g={inlineCode:"code",wrapper:function(e){var r=e.children;return n.createElement(n.Fragment,{},r)}},d=n.forwardRef((function(e,r){var t=e.components,a=e.mdxType,s=e.originalType,l=e.parentName,p=o(e,["components","mdxType","originalType","parentName"]),u=c(t),d=a,y=u["".concat(l,".").concat(d)]||u[d]||g[d]||s;return t?n.createElement(y,i(i({ref:r},p),{},{components:t})):n.createElement(y,i({ref:r},p))}));function y(e,r){var t=arguments,a=r&&r.mdxType;if("string"==typeof e||a){var s=t.length,i=new Array(s);i[0]=d;var o={};for(var l in r)hasOwnProperty.call(r,l)&&(o[l]=r[l]);o.originalType=e,o[u]="string"==typeof e?e:a,i[1]=o;for(var c=2;c<s;c++)i[c]=t[c];return n.createElement.apply(null,i)}return n.createElement.apply(null,t)}d.displayName="MDXCreateElement"},3415:(e,r,t)=>{t.r(r),t.d(r,{assets:()=>l,contentTitle:()=>i,default:()=>g,frontMatter:()=>s,metadata:()=>o,toc:()=>c});var n=t(8168),a=(t(6540),t(5680));const s={id:"rules",label:"Rule Languages",title:"Rule Languages"},i=void 0,o={unversionedId:"reference/dsl/rules",id:"reference/dsl/rules",title:"Rule Languages",description:"Piranha offers support for three distinct rule languages for matching code.",source:"@site/docs/reference/dsl/rules.md",sourceDirName:"reference/dsl",slug:"/reference/dsl/rules",permalink:"/piranha/docs/reference/dsl/rules",draft:!1,editUrl:"https://github.com/uber/piranha/tree/website/site/docs/reference/dsl/rules.md",tags:[],version:"current",frontMatter:{id:"rules",label:"Rule Languages",title:"Rule Languages"},sidebar:"docsSidebar",previous:{title:"Graph Language",permalink:"/piranha/docs/reference/dsl/graphs"},next:{title:"Python API",permalink:"/piranha/docs/reference/api"}},l={},c=[],p={toc:c},u="wrapper";function g(e){let{components:r,...s}=e;return(0,a.yg)(u,(0,n.A)({},p,s,{components:r,mdxType:"MDXLayout"}),(0,a.yg)("p",null,"Piranha offers support for three distinct rule languages for matching code."),(0,a.yg)("h1",{id:"tree-sitter-queries"},"Tree-sitter Queries"),(0,a.yg)("p",null,"The Tree-sitter queries language is one of the rule languages that Piranha supports. For a detailed understanding of the syntax, refer to the ",(0,a.yg)("a",{parentName:"p",href:"https://tree-sitter.github.io/tree-sitter/syntax-highlighting#queries"},"Tree-sitter Syntax Reference"),"."),(0,a.yg)("h1",{id:"regular-expressions-regex"},"Regular Expressions (Regex)"),(0,a.yg)("p",null,"Regex forms another rule language supported by Piranha. To create a rule in regex, prepend your query with ",(0,a.yg)("inlineCode",{parentName:"p"},"rgx "),". For instance: ",(0,a.yg)("inlineCode",{parentName:"p"},"rgx <your regex query>"),". Piranha supports the regex syntax derived from the ",(0,a.yg)("a",{parentName:"p",href:"https://docs.rs/regex/"},"regex")," crate."),(0,a.yg)("h1",{id:"concrete-syntax"},"Concrete Syntax"),(0,a.yg)("p",null,"Piranha's Concrete Syntax is a custom rule language designed for matching and replacing code. Concrete Syntax operates at the parse tree level, similar to ",(0,a.yg)("a",{parentName:"p",href:"https://comby.dev/"},"comby"),", yet with a distinct difference: it will only match a node if it is possible to traverse the corresponding its parse tree from start to finish using the concrete syntax template."),(0,a.yg)("p",null,"Template variables ",(0,a.yg)("inlineCode",{parentName:"p"},":[x], :[y], ...")," are used as placeholders to match arbitrary nodes (i.e., syntactically meaningful constructs)."),(0,a.yg)("p",null,"Consider the following code snippet:"),(0,a.yg)("pre",null,(0,a.yg)("code",{parentName:"pre",className:"language-java"},'exp.isTreated("SHOW_MENU")\n')),(0,a.yg)("p",null,"To match this code snippet, we can use the concrete syntax (cs) template:"),(0,a.yg)("pre",null,(0,a.yg)("code",{parentName:"pre",className:"language-java"},"cs :[object].isTreated(:[string])\n")),(0,a.yg)("p",null,"which matches the method invocation nodes as follows: "),(0,a.yg)("p",null,(0,a.yg)("img",{alt:"example.png",src:t(9292).A,width:"3296",height:"1250"})))}g.isMDXComponent=!0},9292:(e,r,t)=>{t.d(r,{A:()=>n});const n=t.p+"assets/images/example-c5d7e48747b15d7767e2bff93d708be8.png"}}]);
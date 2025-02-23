"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[386],{3080:(e,n,s)=>{s.r(n),s.d(n,{assets:()=>L,contentTitle:()=>E,default:()=>P,frontMatter:()=>I,metadata:()=>l,toc:()=>A});const l=JSON.parse('{"id":"cli/README","title":"Command Line Interface References","description":"The sqlx-ts Command Line Interface (CLI) lets you execute linting from the terminal. The CLI has a variety of options that you can pass to configure sqlx-ts.","source":"@site/docs/cli/README.mdx","sourceDirName":"cli","slug":"/cli/","permalink":"/cli/","draft":false,"unlisted":false,"editUrl":"https://github.com/jasonshin/sqlx-ts/edit/main/docs/cli/README.mdx","tags":[],"version":"current","frontMatter":{},"sidebar":"tutorialSidebar","previous":{"title":"Ignore files","permalink":"/connect/sqlxignore"},"next":{"title":"TypeScript Types Generation","permalink":"/type-generation/"}}');var r=s(4848),a=s(8453),t=s(6540),c=s(4164),i=s(5627),o=s(6347),d=s(372),h=s(604),p=s(1861),u=s(8749);function x(e){return t.Children.toArray(e).filter((e=>"\n"!==e)).map((e=>{if(!e||(0,t.isValidElement)(e)&&function(e){const{props:n}=e;return!!n&&"object"==typeof n&&"value"in n}(e))return e;throw new Error(`Docusaurus error: Bad <Tabs> child <${"string"==typeof e.type?e.type:e.type.name}>: all children of the <Tabs> component should be <TabItem>, and every <TabItem> should have a unique "value" prop.`)}))?.filter(Boolean)??[]}function g(e){const{values:n,children:s}=e;return(0,t.useMemo)((()=>{const e=n??function(e){return x(e).map((e=>{let{props:{value:n,label:s,attributes:l,default:r}}=e;return{value:n,label:s,attributes:l,default:r}}))}(s);return function(e){const n=(0,p.XI)(e,((e,n)=>e.value===n.value));if(n.length>0)throw new Error(`Docusaurus error: Duplicate values "${n.map((e=>e.value)).join(", ")}" found in <Tabs>. Every value needs to be unique.`)}(e),e}),[n,s])}function j(e){let{value:n,tabValues:s}=e;return s.some((e=>e.value===n))}function b(e){let{queryString:n=!1,groupId:s}=e;const l=(0,o.W6)(),r=function(e){let{queryString:n=!1,groupId:s}=e;if("string"==typeof n)return n;if(!1===n)return null;if(!0===n&&!s)throw new Error('Docusaurus error: The <Tabs> component groupId prop is required if queryString=true, because this value is used as the search param name. You can also provide an explicit value such as queryString="my-search-param".');return s??null}({queryString:n,groupId:s});return[(0,h.aZ)(r),(0,t.useCallback)((e=>{if(!r)return;const n=new URLSearchParams(l.location.search);n.set(r,e),l.replace({...l.location,search:n.toString()})}),[r,l])]}function m(e){const{defaultValue:n,queryString:s=!1,groupId:l}=e,r=g(e),[a,c]=(0,t.useState)((()=>function(e){let{defaultValue:n,tabValues:s}=e;if(0===s.length)throw new Error("Docusaurus error: the <Tabs> component requires at least one <TabItem> children component");if(n){if(!j({value:n,tabValues:s}))throw new Error(`Docusaurus error: The <Tabs> has a defaultValue "${n}" but none of its children has the corresponding value. Available values are: ${s.map((e=>e.value)).join(", ")}. If you intend to show no default tab, use defaultValue={null} instead.`);return n}const l=s.find((e=>e.default))??s[0];if(!l)throw new Error("Unexpected error: 0 tabValues");return l.value}({defaultValue:n,tabValues:r}))),[i,o]=b({queryString:s,groupId:l}),[h,p]=function(e){let{groupId:n}=e;const s=function(e){return e?`docusaurus.tab.${e}`:null}(n),[l,r]=(0,u.Dv)(s);return[l,(0,t.useCallback)((e=>{s&&r.set(e)}),[s,r])]}({groupId:l}),x=(()=>{const e=i??h;return j({value:e,tabValues:r})?e:null})();(0,d.A)((()=>{x&&c(x)}),[x]);return{selectedValue:a,selectValue:(0,t.useCallback)((e=>{if(!j({value:e,tabValues:r}))throw new Error(`Can't select invalid tab value=${e}`);c(e),o(e),p(e)}),[o,p,r]),tabValues:r}}var f=s(9136);const v={tabList:"tabList__CuJ",tabItem:"tabItem_LNqP"};function y(e){let{className:n,block:s,selectedValue:l,selectValue:a,tabValues:t}=e;const o=[],{blockElementScrollPositionUntilNextRender:d}=(0,i.a_)(),h=e=>{const n=e.currentTarget,s=o.indexOf(n),r=t[s].value;r!==l&&(d(n),a(r))},p=e=>{let n=null;switch(e.key){case"Enter":h(e);break;case"ArrowRight":{const s=o.indexOf(e.currentTarget)+1;n=o[s]??o[0];break}case"ArrowLeft":{const s=o.indexOf(e.currentTarget)-1;n=o[s]??o[o.length-1];break}}n?.focus()};return(0,r.jsx)("ul",{role:"tablist","aria-orientation":"horizontal",className:(0,c.A)("tabs",{"tabs--block":s},n),children:t.map((e=>{let{value:n,label:s,attributes:a}=e;return(0,r.jsx)("li",{role:"tab",tabIndex:l===n?0:-1,"aria-selected":l===n,ref:e=>{o.push(e)},onKeyDown:p,onClick:h,...a,className:(0,c.A)("tabs__item",v.tabItem,a?.className,{"tabs__item--active":l===n}),children:s??n},n)}))})}function q(e){let{lazy:n,children:s,selectedValue:l}=e;const a=(Array.isArray(s)?s:[s]).filter(Boolean);if(n){const e=a.find((e=>e.props.value===l));return e?(0,t.cloneElement)(e,{className:(0,c.A)("margin-top--md",e.props.className)}):null}return(0,r.jsx)("div",{className:"margin-top--md",children:a.map(((e,n)=>(0,t.cloneElement)(e,{key:n,hidden:e.props.value!==l})))})}function N(e){const n=m(e);return(0,r.jsxs)("div",{className:(0,c.A)("tabs-container",v.tabList),children:[(0,r.jsx)(y,{...n,...e}),(0,r.jsx)(q,{...n,...e})]})}function S(e){const n=(0,f.A)();return(0,r.jsx)(N,{...e,children:x(e.children)},String(n))}const T={tabItem:"tabItem_Ymn6"};function w(e){let{children:n,hidden:s,className:l}=e;return(0,r.jsx)("div",{role:"tabpanel",className:(0,c.A)(T.tabItem,l),hidden:s,children:n})}const I={},E="Command Line Interface References",L={},A=[{value:"Run the CLI",id:"run-the-cli",level:2},{value:"--config",id:"--config",level:3},{value:"--db-host",id:"--db-host",level:3},{value:"--db-name",id:"--db-name",level:3},{value:"--db-port",id:"--db-port",level:3},{value:"--db-type",id:"--db-type",level:3},{value:"--db-user",id:"--db-user",level:3},{value:"--ext",id:"--ext",level:3},{value:"-g, --generate-types",id:"-g---generate-types",level:3},{value:"--generate-path",id:"--generate-path",level:3},{value:"-h, --help",id:"-h---help",level:3},{value:"--ignore",id:"--ignore",level:3},{value:"--log-level",id:"--log-level",level:3},{value:"--pg-search-path",id:"--pg-search-path",level:3},{value:"-V, --version",id:"-v---version",level:3}];function C(e){const n={a:"a",code:"code",h1:"h1",h2:"h2",h3:"h3",header:"header",p:"p",pre:"pre",...(0,a.R)(),...e.components};return(0,r.jsxs)(r.Fragment,{children:[(0,r.jsx)(n.header,{children:(0,r.jsx)(n.h1,{id:"command-line-interface-references",children:"Command Line Interface References"})}),"\n",(0,r.jsxs)(n.p,{children:["The ",(0,r.jsx)(n.code,{children:"sqlx-ts"})," Command Line Interface (CLI) lets you execute linting from the terminal. The CLI has a variety of options that you can pass to configure sqlx-ts."]}),"\n",(0,r.jsx)(n.h2,{id:"run-the-cli",children:"Run the CLI"}),"\n",(0,r.jsxs)(n.p,{children:["sqlx-ts requires Node.js for installation. Follow the instructions in the Getting Started Guide to install ",(0,r.jsx)(n.a,{href:"/installation",children:"sqlx-ts"}),"."]}),"\n",(0,r.jsx)(n.p,{children:"Most users use npx to run sqlx-ts on the command line like this:"}),"\n",(0,r.jsxs)(S,{children:[(0,r.jsx)(w,{value:"npm",label:"npm",default:!0,children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"npx sqlx-ts [options] <PATH>\n"})})}),(0,r.jsx)(w,{value:"yarn",label:"yarn",children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"yarn sqlx-ts [options] <PATH>\n"})})})]}),"\n",(0,r.jsx)(n.p,{children:"such as:"}),"\n",(0,r.jsxs)(S,{children:[(0,r.jsx)(w,{value:"npm",label:"npm",default:!0,children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"# run sqlx-ts to validates SQLs and generate types\nnpx sqlx-ts \\\n    --db-type=postgres \\\n    --db-host=127.0.01 \\\n    --db-port=54321 \\\n    --db-user=postgres \\\n    --db-pass=postgres \\\n    ./src/app -g\n"})})}),(0,r.jsx)(w,{value:"yarn",label:"yarn",children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"# run sqlx-ts to validates SQLs and generate types\nyarn dlx sqlx-ts \\\n    --db-type=postgres \\\n    --db-host=127.0.01 \\\n    --db-port=54321 \\\n    --db-user=postgres \\\n    --db-pass=postgres \\\n    ./src/app -g\n"})})})]}),"\n",(0,r.jsx)(n.p,{children:"or"}),"\n",(0,r.jsxs)(S,{children:[(0,r.jsx)(w,{value:"npm",label:"npm",default:!0,children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"# run sqlx-ts to validates SQLs and generate types\nnpx sqlx-ts --config=.sqlxrc.json ./src/app\n"})})}),(0,r.jsx)(w,{value:"yarn",label:"yarn",children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"# run sqlx-ts to validates SQLs and generate types\nyarn dlx sqlx-ts --config=.sqlxrc.json ./src/app\n"})})})]}),"\n",(0,r.jsx)(n.h1,{id:"options",children:"Options"}),"\n",(0,r.jsxs)(n.p,{children:["You can view all the CLI options by running ",(0,r.jsx)(n.code,{children:"npx sqlx-ts --help"})]}),"\n",(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{children:'A tool for Javascript/Typescript for compile-time safe SQL queries\n\nUSAGE:\nsqlx-ts [OPTIONS] <PATH>\n\n    ARGS:\n<PATH>    Path to the Typescript or Javascript project\n\n    OPTIONS:\n    --config <CONFIG>\n    Path to the file based configuration\n\n    --db-host <DB_HOST>\n    Primary DB host\n\n    --db-name <DB_NAME>\n    Primary DB database name\n\n    --db-pass <DB_PASS>\n    Primary DB pass\n\n    --db-port <DB_PORT>\n    Primary DB Port\n\n    --db-type <DB_TYPE>\n    Type of primary database to connect [possible values: postgres, mysql]\n\n    --db-user <DB_USER>\n    Primary DB user\n\n    --ext <EXT>\n    Javascript Extension [default: ts] [possible values: ts, js]\n\n    -g, --generate-types\n    generate types of raw SQLs using default configuration\n\n    --generate-path <GENERATE_PATH>\n    generates types in a target directory path or a file\n\n    -h, --help\n    Print help information\n\n    --ignore <IGNORE>...\n    Folder paths to ignore\n\n    --log-level <LOG_LEVEL>\n    log level to be used for the CLI debug > info > warn > error [possible values: debug,\n    info, warning, error]\n\n    -m, --message-format <MESSAGE_FORMAT>\n\n\n    --pg-search-path <PG_SEARCH_PATH>\n    PostgreSQL schema search path (default is "$user,public")\n    https://www.postgresql.org/docs/current/ddl-schemas.html#DDL-SCHEMAS-PATH\n\n    -V, --version\n    Print version information\n'})}),"\n",(0,r.jsx)(n.h1,{id:"basic-configuration",children:"Basic Configuration"}),"\n",(0,r.jsx)(n.h3,{id:"--config",children:"--config"}),"\n",(0,r.jsxs)(n.p,{children:["This option allows you to specify an additional configuration file for sqlx-ts. (See ",(0,r.jsx)(n.a,{href:"/connect/config-file",children:"Configuration File"})," for more)"]}),"\n",(0,r.jsxs)(S,{children:[(0,r.jsx)(w,{value:"npm",label:"npm",default:!0,children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"npx sqlx-ts --config=.sqlxrc.json ./src/app\n"})})}),(0,r.jsx)(w,{value:"yarn",label:"yarn",children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"yarn dlx sqlx-ts --config=.sqlxrc.json ./src/app\n"})})})]}),"\n",(0,r.jsx)(n.h3,{id:"--db-host",children:"--db-host"}),"\n",(0,r.jsx)(n.p,{children:"URL to the database that sqlx-ts should point to"}),"\n",(0,r.jsxs)(S,{children:[(0,r.jsx)(w,{value:"npm",label:"npm",default:!0,children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"npx sqlx-ts --config=.sqlxrc.json --db-host=127.0.0.1 ./src/app\n"})})}),(0,r.jsx)(w,{value:"yarn",label:"yarn",children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"yarn dlx sqlx-ts --config=.sqlxrc.json --db-host=127.0.0.1 ./src/app\n"})})})]}),"\n",(0,r.jsx)(n.h3,{id:"--db-name",children:"--db-name"}),"\n",(0,r.jsx)(n.p,{children:"Name of the database that you connect to"}),"\n",(0,r.jsxs)(S,{children:[(0,r.jsx)(w,{value:"npm",label:"npm",default:!0,children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"npx sqlx-ts --config=.sqlxrc.json --db-name=sqlx ./src/app\n"})})}),(0,r.jsx)(w,{value:"yarn",label:"yarn",children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"yarn dlx sqlx-ts --config=.sqlxrc.json --db-name=sqlx ./src/app\n"})})})]}),"\n",(0,r.jsx)(n.h3,{id:"--db-port",children:"--db-port"}),"\n",(0,r.jsx)(n.p,{children:"Port number of the database that you connect to"}),"\n",(0,r.jsxs)(S,{children:[(0,r.jsx)(w,{value:"npm",label:"npm",default:!0,children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"npx sqlx-ts --config=.sqlxrc.json --db-port=3306 ./src/app\n"})})}),(0,r.jsx)(w,{value:"yarn",label:"yarn",children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"yarn dlx sqlx-ts --config=.sqlxrc.json --db-port=3306 ./src/app\n"})})})]}),"\n",(0,r.jsx)(n.h3,{id:"--db-type",children:"--db-type"}),"\n",(0,r.jsxs)(n.p,{children:["Type of the database that you connect to. It can be either ",(0,r.jsx)(n.code,{children:"mysql"})," or ",(0,r.jsx)(n.code,{children:"postgres"})]}),"\n",(0,r.jsxs)(S,{children:[(0,r.jsx)(w,{value:"npm",label:"npm",default:!0,children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"npx sqlx-ts --config=.sqlxrc.json --db-type=postgres ./src/app\n"})})}),(0,r.jsx)(w,{value:"yarn",label:"yarn",children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"yarn dlx sqlx-ts --config=.sqlxrc.json --db-type=postgres ./src/app\n"})})})]}),"\n",(0,r.jsx)(n.h3,{id:"--db-user",children:"--db-user"}),"\n",(0,r.jsx)(n.p,{children:"user of the database that you connect to"}),"\n",(0,r.jsxs)(S,{children:[(0,r.jsx)(w,{value:"npm",label:"npm",default:!0,children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"npx sqlx-ts --config=.sqlxrc.json --db-user=postgres ./src/app\n"})})}),(0,r.jsx)(w,{value:"yarn",label:"yarn",children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"yarn dlx sqlx-ts --config=.sqlxrc.json --db-user=postgres ./src/app\n"})})})]}),"\n",(0,r.jsx)(n.h3,{id:"--ext",children:"--ext"}),"\n",(0,r.jsxs)(n.p,{children:["File extensions to search. It can be either ",(0,r.jsx)(n.code,{children:"ts"})," | ",(0,r.jsx)(n.code,{children:"js"})," - [default: ts]"]}),"\n",(0,r.jsxs)(S,{children:[(0,r.jsx)(w,{value:"npm",label:"npm",default:!0,children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"npx sqlx-ts --config=.sqlxrc.json --ext=ts ./src/app\n"})})}),(0,r.jsx)(w,{value:"yarn",label:"yarn",children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"yarn dlx sqlx-ts --config=.sqlxrc.json --ext=ts ./src/app\n"})})})]}),"\n",(0,r.jsx)(n.h3,{id:"-g---generate-types",children:"-g, --generate-types"}),"\n",(0,r.jsx)(n.p,{children:"generate type definitions of SQLs detected by sqlx-ts"}),"\n",(0,r.jsxs)(S,{children:[(0,r.jsx)(w,{value:"npm",label:"npm",default:!0,children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"npx sqlx-ts --config=.sqlxrc.json ./src/app -g\n"})})}),(0,r.jsx)(w,{value:"yarn",label:"yarn",children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"yarn dlx sqlx-ts --config=.sqlxrc.json ./src/app -g\n"})})})]}),"\n",(0,r.jsx)(n.h3,{id:"--generate-path",children:"--generate-path"}),"\n",(0,r.jsx)(n.p,{children:"File path to generate types"}),"\n",(0,r.jsxs)(S,{children:[(0,r.jsx)(w,{value:"npm",label:"npm",default:!0,children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"npx sqlx-ts --config=.sqlxrc.json --generate-path=src/app/queries.ts ./src/app\n"})})}),(0,r.jsx)(w,{value:"yarn",label:"yarn",children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"yarn dlx sqlx-ts --config=.sqlxrc.json --generate-path=src/app/queries.ts ./src/app\n"})})})]}),"\n",(0,r.jsx)(n.h3,{id:"-h---help",children:"-h, --help"}),"\n",(0,r.jsx)(n.p,{children:"Shows all sqlx-ts CLI options"}),"\n",(0,r.jsxs)(S,{children:[(0,r.jsx)(w,{value:"npm",label:"npm",default:!0,children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"npx sqlx-ts --help\n"})})}),(0,r.jsx)(w,{value:"yarn",label:"yarn",children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"yarn dlx sqlx-ts --help\n"})})})]}),"\n",(0,r.jsx)(n.h3,{id:"--ignore",children:"--ignore"}),"\n",(0,r.jsxs)(n.p,{children:["File/Folder path patterns to ignore. The additional ignore patterns provided by this flag will\nbe added to the base ignore patters (",(0,r.jsx)(n.code,{children:"*.queries.ts"})," and ",(0,r.jsx)(n.code,{children:"*.queries.js"})," also the patterns provided by ",(0,r.jsx)(n.a,{href:"/connect/sqlxignore",children:".sqlxrcignore file"}),")"]}),"\n",(0,r.jsxs)(S,{children:[(0,r.jsx)(w,{value:"npm",label:"npm",default:!0,children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"npx sqlx-ts --ignore=src/app/select.ts --ignore=src/app/stuff --ignore src/app/select.*.ts\n"})})}),(0,r.jsx)(w,{value:"yarn",label:"yarn",children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"yarn dlx sqlx-ts --ignore=src/app/select.ts --ignore=src/app/stuff --ignore src/app/select.*.ts\n"})})})]}),"\n",(0,r.jsx)(n.h3,{id:"--log-level",children:"--log-level"}),"\n",(0,r.jsxs)(n.p,{children:["log level to be used for the CLI - level follows in this order: ",(0,r.jsx)(n.code,{children:"debug > info > warning > error"})," [possible values: debug, info, warning, error].\n(default is ",(0,r.jsx)(n.code,{children:"info"}),")"]}),"\n",(0,r.jsxs)(S,{children:[(0,r.jsx)(w,{value:"npm",label:"npm",default:!0,children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"npx sqlx-ts --ignore=src/app/select.ts --log-level=error\n"})})}),(0,r.jsx)(w,{value:"yarn",label:"yarn",children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"yarn dlx sqlx-ts --ignore=src/app/select.ts --log-level=error\n"})})})]}),"\n",(0,r.jsx)(n.h3,{id:"--pg-search-path",children:"--pg-search-path"}),"\n",(0,r.jsxs)(n.p,{children:['PostgreSQL schema search path (default is "$user,public")\n',(0,r.jsx)(n.a,{href:"https://www.postgresql.org/docs/current/ddl-schemas.html#DDL-SCHEMAS-PATH",children:"https://www.postgresql.org/docs/current/ddl-schemas.html#DDL-SCHEMAS-PATH"})]}),"\n",(0,r.jsxs)(S,{children:[(0,r.jsx)(w,{value:"npm",label:"npm",default:!0,children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"npx sqlx-ts --ignore=src/app/select.ts --pg-search-path=public,myschema\n"})})}),(0,r.jsx)(w,{value:"yarn",label:"yarn",children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"yarn dlx sqlx-ts --ignore=src/app/select.ts --pg-search-path=public,myschema\n"})})})]}),"\n",(0,r.jsx)(n.h3,{id:"-v---version",children:"-V, --version"}),"\n",(0,r.jsx)(n.p,{children:"Prints version information of sqlx-ts CLI"}),"\n",(0,r.jsxs)(S,{children:[(0,r.jsx)(w,{value:"npm",label:"npm",default:!0,children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"npx sqlx-ts --version\n"})})}),(0,r.jsx)(w,{value:"yarn",label:"yarn",children:(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-bash",children:"yarn dlx sqlx-ts --version\n"})})})]})]})}function P(e={}){const{wrapper:n}={...(0,a.R)(),...e.components};return n?(0,r.jsx)(n,{...e,children:(0,r.jsx)(C,{...e})}):C(e)}},8453:(e,n,s)=>{s.d(n,{R:()=>t,x:()=>c});var l=s(6540);const r={},a=l.createContext(r);function t(e){const n=l.useContext(a);return l.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function c(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(r):e.components||r:t(e.components),l.createElement(a.Provider,{value:n},e.children)}}}]);
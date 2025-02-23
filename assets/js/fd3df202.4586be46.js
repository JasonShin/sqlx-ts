"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[826],{8453:(e,t,n)=>{n.d(t,{R:()=>o,x:()=>i});var s=n(6540);const r={},a=s.createContext(r);function o(e){const t=s.useContext(a);return s.useMemo((function(){return"function"==typeof e?e(t):{...t,...e}}),[t,e])}function i(e){let t;return t=e.disableParentContext?"function"==typeof e.components?e.components(r):e.components||r:o(e.components),s.createElement(a.Provider,{value:t},e.children)}},9153:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>l,contentTitle:()=>i,default:()=>d,frontMatter:()=>o,metadata:()=>s,toc:()=>p});const s=JSON.parse('{"id":"type-generation/update","title":"UPDATE statements","description":"To read more about how SQLX-TS translates query parameters, visit this page","source":"@site/docs/type-generation/update.md","sourceDirName":"type-generation","slug":"/type-generation/update","permalink":"/sqlx-ts/type-generation/update","draft":false,"unlisted":false,"editUrl":"https://github.com/jasonshin/sqlx-ts/edit/main/book/docs/type-generation/update.md","tags":[],"version":"current","sidebarPosition":3,"frontMatter":{"sidebar_position":3},"sidebar":"tutorialSidebar","previous":{"title":"INSERT statements","permalink":"/sqlx-ts/type-generation/insert"},"next":{"title":"DELETE statement","permalink":"/sqlx-ts/type-generation/delete"}}');var r=n(4848),a=n(8453);const o={sidebar_position:3},i="UPDATE statements",l={},p=[{value:"MySQL",id:"mysql",level:2},{value:"PostgreSQL",id:"postgresql",level:2}];function u(e){const t={a:"a",code:"code",h1:"h1",h2:"h2",header:"header",p:"p",pre:"pre",...(0,a.R)(),...e.components};return(0,r.jsxs)(r.Fragment,{children:[(0,r.jsx)(t.header,{children:(0,r.jsx)(t.h1,{id:"update-statements",children:"UPDATE statements"})}),"\n",(0,r.jsxs)(t.p,{children:["To read more about how SQLX-TS translates query parameters, ",(0,r.jsx)(t.a,{href:"/type-generation#capabilities",children:"visit this page"})]}),"\n",(0,r.jsx)(t.h2,{id:"mysql",children:"MySQL"}),"\n",(0,r.jsx)(t.p,{children:"Query params within an update statement can be converted into TypeScript types as well"}),"\n",(0,r.jsx)(t.pre,{children:(0,r.jsx)(t.code,{className:"language-typescript",children:"const someQuery = sql`\nUPDATE items\nJOIN tables ON tables.id = items.table_id\nSET items.food_type = ?\nWHERE tables.id = ?\n`\n"})}),"\n",(0,r.jsx)(t.p,{children:"would generate following"}),"\n",(0,r.jsx)(t.pre,{children:(0,r.jsx)(t.code,{className:"language-typescript",children:"export type SomeQueryParams = [string, number]\n\nexport type SomeQueryResult = number\n\nexport interface ISomeQueryQuery {\n    params: SomeQueryParams\n    result: SomeQueryResult\n}\n"})}),"\n",(0,r.jsx)(t.h2,{id:"postgresql",children:"PostgreSQL"}),"\n",(0,r.jsx)(t.p,{children:"As query params of PostgresSQL uses the numbered parameters, it's meaningless to generate a nested array respresnetation of them."}),"\n",(0,r.jsx)(t.p,{children:"If you have"}),"\n",(0,r.jsx)(t.pre,{children:(0,r.jsx)(t.code,{className:"language-typescript",children:"const someQuery = sql`\nUPDATE items\nJOIN tables ON tables.id = items.table_id\nSET items.food_type = $2\nWHERE tables.id = $1\n`\n"})}),"\n",(0,r.jsx)(t.p,{children:"Above query will generate the following typings"}),"\n",(0,r.jsx)(t.pre,{children:(0,r.jsx)(t.code,{className:"language-typescript",children:"export type SomePostgresInputQueryParams = [string, number, number, number, number, number, number];\n\nexport interface ISomePostgresInputQueryResult {\n    \n};\n\nexport interface ISomePostgresInputQueryQuery {\n    params: SomePostgresInputQueryParams;\n    result: ISomePostgresInputQueryResult;\n};\n"})})]})}function d(e={}){const{wrapper:t}={...(0,a.R)(),...e.components};return t?(0,r.jsx)(t,{...e,children:(0,r.jsx)(u,{...e})}):u(e)}}}]);
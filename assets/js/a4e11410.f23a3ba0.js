"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[90],{8381:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>l,contentTitle:()=>o,default:()=>u,frontMatter:()=>a,metadata:()=>s,toc:()=>p});const s=JSON.parse('{"id":"type-generation/insert","title":"INSERT statements","description":"To read more about how SQLX-TS translates query parameters, visit this page","source":"@site/docs/type-generation/insert.md","sourceDirName":"type-generation","slug":"/type-generation/insert","permalink":"/sqlx-ts/type-generation/insert","draft":false,"unlisted":false,"editUrl":"https://github.com/jasonshin/sqlx-ts/edit/main/book/docs/type-generation/insert.md","tags":[],"version":"current","sidebarPosition":2,"frontMatter":{"sidebar_position":2},"sidebar":"tutorialSidebar","previous":{"title":"SELECT statements","permalink":"/sqlx-ts/type-generation/select"},"next":{"title":"UPDATE statements","permalink":"/sqlx-ts/type-generation/update"}}');var r=t(4848),i=t(8453);const a={sidebar_position:2},o="INSERT statements",l={},p=[{value:"MySQL",id:"mysql",level:2},{value:"PostgreSQL",id:"postgresql",level:2}];function c(e){const n={a:"a",code:"code",h1:"h1",h2:"h2",header:"header",p:"p",pre:"pre",...(0,i.R)(),...e.components};return(0,r.jsxs)(r.Fragment,{children:[(0,r.jsx)(n.header,{children:(0,r.jsx)(n.h1,{id:"insert-statements",children:"INSERT statements"})}),"\n",(0,r.jsxs)(n.p,{children:["To read more about how SQLX-TS translates query parameters, ",(0,r.jsx)(n.a,{href:"/type-generation#capabilities",children:"visit this page"})]}),"\n",(0,r.jsx)(n.h2,{id:"mysql",children:"MySQL"}),"\n",(0,r.jsx)(n.p,{children:"Query params within an insert statement can be converted into TypeScript types as well"}),"\n",(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-typescript",children:"const someQuery = sql`\nINSERT INTO items (id, food_type, time_takes_to_cook, table_id, points)\nVALUES\n    (?, ?, ?, ?, ?),\n    (?, ?, ?, ?, ?);\n`\n"})}),"\n",(0,r.jsx)(n.p,{children:"would generate following"}),"\n",(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-typescript",children:"export type SomeQueryParams = [\n    [number, string, number, number, number],\n    [number, string, number, number, number]\n]\n\nexport interface ISomeQueryQuery {\n    params: SomeQueryParams\n    result: null\n}\n"})}),"\n",(0,r.jsx)(n.p,{children:"it also supports type generation for RETURNING statement"}),"\n",(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-typescript",children:"const insertWildcard = sql`\nINSERT INTO items\nVALUES (1, 'sword', 'epic', 'test', null)\nRETURNING *;\n`\n"})}),"\n",(0,r.jsx)(n.p,{children:"generates the following result type"}),"\n",(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-typescript",children:"export interface IInsertWildcardResult {\n    flavorText: string | null;\n    id: number;\n    inventoryId: number | null;\n    name: string;\n    rarity: string | null;\n};\n"})}),"\n",(0,r.jsx)(n.h2,{id:"postgresql",children:"PostgreSQL"}),"\n",(0,r.jsx)(n.p,{children:"As query params of PostgresSQL uses the numbered parameters, it's meaningless to generate a nested array respresnetation of them."}),"\n",(0,r.jsx)(n.p,{children:"If you have"}),"\n",(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-typescript",children:"const somePostgresInputQuery = sql`\nINSERT INTO items (id, food_type, time_takes_to_cook, table_id, points)\nVALUES\n($2, $1, 2, $3, 2),\n($5, 'test', $4, $7, $6);\n`\n"})}),"\n",(0,r.jsx)(n.p,{children:"Above query will generate the following typings"}),"\n",(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-typescript",children:"export type SomePostgresInputQueryParams = [string, number, number, number, number, number, number];\n\nexport interface ISomePostgresInputQueryResult {\n    \n};\n\nexport interface ISomePostgresInputQueryQuery {\n    params: SomePostgresInputQueryParams;\n    result: ISomePostgresInputQueryResult;\n};\n"})})]})}function u(e={}){const{wrapper:n}={...(0,i.R)(),...e.components};return n?(0,r.jsx)(n,{...e,children:(0,r.jsx)(c,{...e})}):c(e)}},8453:(e,n,t)=>{t.d(n,{R:()=>a,x:()=>o});var s=t(6540);const r={},i=s.createContext(r);function a(e){const n=s.useContext(i);return s.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function o(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(r):e.components||r:a(e.components),s.createElement(i.Provider,{value:n},e.children)}}}]);
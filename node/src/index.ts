export function sql<T extends TemplateStringsArray>(query: T): string {
  return query[0];
}
console.log('test')
const x = sql`a`

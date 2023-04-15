export function sql<T extends TemplateStringsArray>(query: T): string {
  return query[0];
}

const x = sql`a`

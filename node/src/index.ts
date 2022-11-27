export const sql = (query: TemplateStringsArray) => {
  if (query[0] === undefined || query[0] === null) {
    return null;
  }

  return query[0];
};

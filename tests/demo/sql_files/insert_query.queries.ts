export type InsertQueryParams = [string, string | null];

export interface IInsertQueryResult {
	id: number;
	name: string;
}

export interface IInsertQueryQuery {
	params: InsertQueryParams;
	result: IInsertQueryResult;
}

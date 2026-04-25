export type SomeQueryParams = [];

export interface ISomeQueryResult {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
}

export interface ISomeQueryQuery {
	params: SomeQueryParams;
	result: ISomeQueryResult;
}

export type TestInsertParams = [number, string];

export interface ITestInsertResult {
	
}

export interface ITestInsertQuery {
	params: TestInsertParams;
	result: ITestInsertResult;
}

export type TestUpdateParams = [string | null];

export interface ITestUpdateResult {
	
}

export interface ITestUpdateQuery {
	params: TestUpdateParams;
	result: ITestUpdateResult;
}

export type TestDeleteParams = [string | null];

export interface ITestDeleteResult {
	
}

export interface ITestDeleteQuery {
	params: TestDeleteParams;
	result: ITestDeleteResult;
}

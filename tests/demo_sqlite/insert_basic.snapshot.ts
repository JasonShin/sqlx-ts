export type InsertItemParams = [string, string | null, string | null, number | null];

export interface IInsertItemResult {
	
}

export interface IInsertItemQuery {
	params: InsertItemParams;
	result: IInsertItemResult;
}

export type InsertCharacterParams = [string, number | null, number | null, number | null];

export interface IInsertCharacterResult {
	
}

export interface IInsertCharacterQuery {
	params: InsertCharacterParams;
	result: IInsertCharacterResult;
}


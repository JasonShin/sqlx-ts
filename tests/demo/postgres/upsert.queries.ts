export type UpsertDoNothingParams = [number, string, string | null];

export interface IUpsertDoNothingResult {
	
}

export interface IUpsertDoNothingQuery {
	params: UpsertDoNothingParams;
	result: IUpsertDoNothingResult;
}

export type UpsertDoUpdateParams = [number, string, string | null];

export interface IUpsertDoUpdateResult {
	
}

export interface IUpsertDoUpdateQuery {
	params: UpsertDoUpdateParams;
	result: IUpsertDoUpdateResult;
}

export type UpsertWithWhereParams = [number, string, string | null];

export interface IUpsertWithWhereResult {
	
}

export interface IUpsertWithWhereQuery {
	params: UpsertWithWhereParams;
	result: IUpsertWithWhereResult;
}

export type UpsertWithReturningParams = [number, string, string | null];

export interface IUpsertWithReturningResult {
	flavorText: string | null;
	id: number;
	inventoryId: number | null;
	name: string;
	rarity: string | null;
}

export interface IUpsertWithReturningQuery {
	params: UpsertWithReturningParams;
	result: IUpsertWithReturningResult;
}

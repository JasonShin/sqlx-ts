export type UpdateReturningAllParams = [string, number];

export interface IUpdateReturningAllResult {
	flavorText: string | null;
	id: number;
	inventoryId: number | null;
	name: string;
	rarity: string | null;
}

export interface IUpdateReturningAllQuery {
	params: UpdateReturningAllParams;
	result: IUpdateReturningAllResult;
}

export type UpdateReturningSpecificParams = [string, number];

export interface IUpdateReturningSpecificResult {
	id: number;
	name: string;
}

export interface IUpdateReturningSpecificQuery {
	params: UpdateReturningSpecificParams;
	result: IUpdateReturningSpecificResult;
}

export type UpdateReturningWithAliasParams = [string, number];

export interface IUpdateReturningWithAliasResult {
	updatedId: number;
	updatedName: string;
}

export interface IUpdateReturningWithAliasQuery {
	params: UpdateReturningWithAliasParams;
	result: IUpdateReturningWithAliasResult;
}

export type UpdateReturningExpressionParams = [string, number];

export interface IUpdateReturningExpressionResult {
	id: number;
	lowerName: string;
}

export interface IUpdateReturningExpressionQuery {
	params: UpdateReturningExpressionParams;
	result: IUpdateReturningExpressionResult;
}

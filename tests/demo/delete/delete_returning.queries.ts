export type DeleteReturningAllParams = [number];

export interface IDeleteReturningAllResult {
	flavorText: string | null;
	id: number;
	inventoryId: number | null;
	name: string;
	rarity: string | null;
}

export interface IDeleteReturningAllQuery {
	params: DeleteReturningAllParams;
	result: IDeleteReturningAllResult;
}

export type DeleteReturningSpecificParams = [number];

export interface IDeleteReturningSpecificResult {
	id: number;
	name: string;
}

export interface IDeleteReturningSpecificQuery {
	params: DeleteReturningSpecificParams;
	result: IDeleteReturningSpecificResult;
}

export type DeleteReturningWithAliasParams = [number];

export interface IDeleteReturningWithAliasResult {
	deletedId: number;
	deletedName: string;
}

export interface IDeleteReturningWithAliasQuery {
	params: DeleteReturningWithAliasParams;
	result: IDeleteReturningWithAliasResult;
}

export type DeleteReturningExpressionParams = [number];

export interface IDeleteReturningExpressionResult {
	id: number;
	upperName: string;
}

export interface IDeleteReturningExpressionQuery {
	params: DeleteReturningExpressionParams;
	result: IDeleteReturningExpressionResult;
}

export type BacktickTableNameParams = [];

export interface IBacktickTableNameResult {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
}

export interface IBacktickTableNameQuery {
	params: BacktickTableNameParams;
	result: IBacktickTableNameResult;
}

export type BacktickQualifiedNameParams = [];

export interface IBacktickQualifiedNameResult {
	flavor_text: string | null;
	id: number;
	inventory_id: number | null;
	name: string;
	rarity: string | null;
}

export interface IBacktickQualifiedNameQuery {
	params: BacktickQualifiedNameParams;
	result: IBacktickQualifiedNameResult;
}

export type BacktickColumnNamesParams = [number];

export interface IBacktickColumnNamesResult {
	id: number;
	name: string;
}

export interface IBacktickColumnNamesQuery {
	params: BacktickColumnNamesParams;
	result: IBacktickColumnNamesResult;
}

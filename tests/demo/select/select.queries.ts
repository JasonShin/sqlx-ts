export type SelectSql1Params = [];

export interface ISelectSql1Result {
	description: string | null;
	food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};

export interface ISelectSql1Query {
	params: SelectSql1Params;
	result: ISelectSql1Result;
};

export type SelectSql2Params = [];

export interface ISelectSql2Result {
	description: string | null;
	food_type: string;
	id: number;
	number: number;
	occupied: boolean;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};

export interface ISelectSql2Query {
	params: SelectSql2Params;
	result: ISelectSql2Result;
};

export type SelectSql3Params = [];

export interface ISelectSql3Result {
	tableNumber: number;
};

export interface ISelectSql3Query {
	params: SelectSql3Params;
	result: ISelectSql3Result;
};

export type SelectSql4Params = [];

export interface ISelectSql4Result {
	description: string | null;
	food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};

export interface ISelectSql4Query {
	params: SelectSql4Params;
	result: ISelectSql4Result;
};

export type SelectSql5Params = [];

export interface ISelectSql5Result {
	description: string | null;
	food_type: string;
	id: number;
	number: number;
	occupied: boolean;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};

export interface ISelectSql5Query {
	params: SelectSql5Params;
	result: ISelectSql5Result;
};

export type SelectSql6Params = [number, number];

export interface ISelectSql6Result {
	id: number;
};

export interface ISelectSql6Query {
	params: SelectSql6Params;
	result: ISelectSql6Result;
};

export type SelectSql9Params = [boolean];

export interface ISelectSql9Result {
	description: string | null;
	food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};

export interface ISelectSql9Query {
	params: SelectSql9Params;
	result: ISelectSql9Result;
};

export type SelectSql10Params = [];

export interface ISelectSql10Result {
	id: number;
	number: number;
	occupied: boolean;
};

export interface ISelectSql10Query {
	params: SelectSql10Params;
	result: ISelectSql10Result;
};

export type SelectSql11Params = [string, string];

export interface ISelectSql11Result {
	hmm: any;
	id: number;
	number: number;
};

export interface ISelectSql11Query {
	params: SelectSql11Params;
	result: ISelectSql11Result;
};

export type SelectSql12Params = [number];

export interface ISelectSql12Result {
	id: number;
};

export interface ISelectSql12Query {
	params: SelectSql12Params;
	result: ISelectSql12Result;
};

export type SelectSql13Params = [string];

export interface ISelectSql13Result {
	description: string | null;
	food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};

export interface ISelectSql13Query {
	params: SelectSql13Params;
	result: ISelectSql13Result;
};

export type SelectSql14Params = [number];

export interface ISelectSql14Result {
	description: string | null;
	food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};

export interface ISelectSql14Query {
	params: SelectSql14Params;
	result: ISelectSql14Result;
};

export type SelectSql15Params = [];

export interface ISelectSql15Result {
	id2: number;
	itemsId: number;
};

export interface ISelectSql15Query {
	params: SelectSql15Params;
	result: ISelectSql15Result;
};

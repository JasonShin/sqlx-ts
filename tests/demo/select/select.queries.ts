

export type Sql1Params = [];


export interface ISql1Result {
    food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};


export interface ISql1Query {
    params: Sql1Params;
    result: ISql1Result;
};




export type Sql2Params = [];


export interface ISql2Result {
    food_type: string;
	id: number;
	number: number;
	occupied: boolean;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};


export interface ISql2Query {
    params: Sql2Params;
    result: ISql2Result;
};




export type Sql3Params = [];


export interface ISql3Result {
    tableNumber: number;
};


export interface ISql3Query {
    params: Sql3Params;
    result: ISql3Result;
};




export type Sql4Params = [];


export interface ISql4Result {
    food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};


export interface ISql4Query {
    params: Sql4Params;
    result: ISql4Result;
};




export type Sql5Params = [];


export interface ISql5Result {
    food_type: string;
	id: number;
	number: number;
	occupied: boolean;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};


export interface ISql5Query {
    params: Sql5Params;
    result: ISql5Result;
};




export type Sql6Params = [number, number];


export interface ISql6Result {
    id: number;
};


export interface ISql6Query {
    params: Sql6Params;
    result: ISql6Result;
};




export type Sql9Params = [boolean];


export interface ISql9Result {
    food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};


export interface ISql9Query {
    params: Sql9Params;
    result: ISql9Result;
};




export type Sql10Params = [];


export interface ISql10Result {
    id: number;
	number: number;
	occupied: boolean;
};


export interface ISql10Query {
    params: Sql10Params;
    result: ISql10Result;
};




export type Sql11Params = [string, string];


export interface ISql11Result {
    hmm: any;
	id: number;
	number: number;
};


export interface ISql11Query {
    params: Sql11Params;
    result: ISql11Result;
};




export type Sql12Params = [number];


export interface ISql12Result {
    id: number;
};


export interface ISql12Query {
    params: Sql12Params;
    result: ISql12Result;
};




export type Sql13Params = [string];


export interface ISql13Result {
    food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};


export interface ISql13Query {
    params: Sql13Params;
    result: ISql13Result;
};




export type Sql14Params = [number];


export interface ISql14Result {
    food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};


export interface ISql14Query {
    params: Sql14Params;
    result: ISql14Result;
};


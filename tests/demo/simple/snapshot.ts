

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



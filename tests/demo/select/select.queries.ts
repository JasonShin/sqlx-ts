

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


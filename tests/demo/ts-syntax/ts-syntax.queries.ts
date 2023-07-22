

export type GetItems2Params = [];


export interface IGetItems2Result {
    food_type: string;
	id: number;
	points: number;
	table_id: number;
	time_takes_to_cook: number;
};


export interface IGetItems2Query {
    params: GetItems2Params;
    result: IGetItems2Result;
};


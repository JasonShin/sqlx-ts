

export type SubQuery2Params = [number];


export interface ISubQuery2Result {
    id: number;
	points: number;
};


export interface ISubQuery2Query {
    params: SubQuery2Params;
    result: ISubQuery2Result;
};


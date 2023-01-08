

export type SubQuery1Params = [number];


export interface ISubQuery1Result {
    id: number;
	points: number;
};


export interface ISubQuery1Query {
    params: SubQuery1Params;
    result: ISubQuery1Result;
};




export type SubQuery2Params = [number, string, number];


export interface ISubQuery2Result {
    id: number;
	points: number;
};


export interface ISubQuery2Query {
    params: SubQuery2Params;
    result: ISubQuery2Result;
};




export type SubQuery1Params = [number];


export interface ISubQuery1Result {
    id: number;
	points: number;
};


export interface ISubQuery1Query {
    params: SubQuery1Params;
    result: ISubQuery1Result;
};


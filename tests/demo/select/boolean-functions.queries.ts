

export type ExistsParams = [number];


export interface IExistsResult {
    exists: boolean;
};


export interface IExistsQuery {
    params: ExistsParams;
    result: IExistsResult;
};




export type IsTrueParams = [];


export interface IIsTrueResult {
    isTrue: boolean;
};


export interface IIsTrueQuery {
    params: IsTrueParams;
    result: IIsTrueResult;
};




export type IsFalseParams = [];


export interface IIsFalseResult {
    isFalse: boolean;
};


export interface IIsFalseQuery {
    params: IsFalseParams;
    result: IIsFalseResult;
};




export type InListParams = [];


export interface IInListResult {
    test: boolean;
};


export interface IInListQuery {
    params: InListParams;
    result: IInListResult;
};


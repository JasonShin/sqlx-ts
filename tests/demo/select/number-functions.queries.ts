

export type CeilParams = [];


export interface ICeilResult {
    id: number;
	someNumber: number;
};


export interface ICeilQuery {
    params: CeilParams;
    result: ICeilResult;
};




export type AllParams = [];


export interface IAllResult {
    abs1: number;
	acos1: number;
	asin1: number;
	atan1: number;
	avg1: number;
	ceil1: number;
	ceiling1: number;
	cos1: number;
	cot1: number;
	count1: number;
	degrees1: number;
	exp1: number;
	floor1: number;
	greatest1: number;
	least1: number;
	ln1: number;
	log1: number;
	log101: number;
	max1: number;
	min1: number;
	mod1: number;
	pi1: number;
	pow1: number;
	pow2: number;
	radians1: number;
	round1: number;
	sign1: number;
	sin1: number;
	sqrt1: number;
	sum1: number;
	tan1: number;
	trunc1: number;
};


export interface IAllQuery {
    params: AllParams;
    result: IAllResult;
};


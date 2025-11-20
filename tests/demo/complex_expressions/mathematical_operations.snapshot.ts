export type ArithmeticOperatorsParams = [];

export interface IArithmeticOperatorsResult {
	id: number;
	idDivided2: number;
	idMinus5: number;
	idMod3: number;
	idPlus10: number;
	idTimes2: number;
	name: string;
}

export interface IArithmeticOperatorsQuery {
	params: ArithmeticOperatorsParams;
	result: IArithmeticOperatorsResult;
}

export type MathFunctionsParams = [];

export interface IMathFunctionsResult {
	distanceFrom10: number;
	id: number;
	idDiv3Ceil: number;
	idDiv3Floor: number;
	idDiv3Rounded: number;
	idSqrt: number;
	idSquared: number;
	name: string;
}

export interface IMathFunctionsQuery {
	params: MathFunctionsParams;
	result: IMathFunctionsResult;
}

export type ComparisonOperatorsParams = [];

export interface IComparisonOperatorsResult {
	id: number;
	isAbove5: number;
	isAtLeast5: number;
	isAtMost5: number;
	isBelow5: number;
	isExactly5: number;
	isNot5: number;
	name: string;
}

export interface IComparisonOperatorsQuery {
	params: ComparisonOperatorsParams;
	result: IComparisonOperatorsResult;
}

export type BetweenAndInParams = [];

export interface IBetweenAndInResult {
	id: number;
	isCommonOrRare: boolean;
	name: string;
	rarity: string | null;
}

export interface IBetweenAndInQuery {
	params: BetweenAndInParams;
	result: IBetweenAndInResult;
}


/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { Attempt } from './Attempt';
import type { ClimbType } from './ClimbType';
import type { Scale } from './Scale';
import type { Style } from './Style';
export type ClimbData = {
    attempt: Attempt;
    climb_type: ClimbType;
    grade: string;
    notes?: string | null;
    pointer?: string | null;
    scale: Scale;
    style?: Style | null;
};


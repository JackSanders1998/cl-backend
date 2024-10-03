/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { Attempt } from './Attempt';
import type { ClimbType } from './ClimbType';
import type { Scale } from './Scale';
import type { Style } from './Style';
export type UpdateClimb = {
    attempt?: Attempt | null;
    climb_type?: ClimbType | null;
    grade?: string | null;
    notes?: string | null;
    pointer?: string | null;
    scale?: Scale | null;
    sesh_id?: string | null;
    style?: Style | null;
};


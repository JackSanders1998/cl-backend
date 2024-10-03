/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { Attempt } from './Attempt';
import type { ClimbType } from './ClimbType';
import type { Scale } from './Scale';
import type { Style } from './Style';
export type Climb = {
    attempt: Attempt;
    climb_id: string;
    climb_type: ClimbType;
    created_at: string;
    grade: string;
    notes?: string | null;
    pointer?: string | null;
    scale: Scale;
    sesh_id: string;
    style?: Style | null;
    updated_at: string;
};


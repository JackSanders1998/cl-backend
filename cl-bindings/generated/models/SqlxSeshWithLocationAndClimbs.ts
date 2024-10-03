/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { Attempt } from './Attempt';
import type { ClimbType } from './ClimbType';
import type { Scale } from './Scale';
import type { Style } from './Style';
export type SqlxSeshWithLocationAndClimbs = {
    attempt: Attempt;
    climb_notes?: string | null;
    climb_type: ClimbType;
    created_at: string;
    end?: string | null;
    environment: string;
    grade: string;
    location_id: string;
    name: string;
    notes?: string | null;
    pointer?: string | null;
    scale: Scale;
    sesh_id: string;
    start: string;
    style?: Style | null;
    updated_at: string;
    user_id: string;
};


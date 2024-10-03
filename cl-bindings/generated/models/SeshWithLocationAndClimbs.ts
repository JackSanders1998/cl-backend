/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ClimbData } from './ClimbData';
import type { CreateLocation } from './CreateLocation';
export type SeshWithLocationAndClimbs = {
    climbs: Array<ClimbData>;
    created_at: string;
    end?: string | null;
    location: CreateLocation;
    location_id: string;
    notes?: string | null;
    sesh_id: string;
    start: string;
    updated_at: string;
    user_id: string;
};


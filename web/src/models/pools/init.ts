import { $pools, findPoolsFx } from '.';

import './effects';

$pools.on(findPoolsFx.doneData, (_, newPools) => newPools);

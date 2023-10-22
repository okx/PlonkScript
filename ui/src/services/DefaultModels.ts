// import o0 from '../assets/output.json';
import o1 from '../assets/simple_example.json';
import o2 from '../assets/fib1.json';
import o3 from '../assets/fib2.json';
import o4 from '../assets/fib3.json';
import o5 from '../assets/fib4.json';
import o6 from '../assets/range_check1.json';
import o7 from '../assets/range_check2.json';
import o8 from '../assets/range_check3_broken.json';
import o9 from '../assets/merkle_path_poseidon.json';
import { MockProverData } from './ConstraintSystem';

export interface IDataModel {
  name: string;
  data: MockProverData;
  title?: string;
  description?: string;
  sourceUrl?: string;
}

export const dataList: IDataModel[] = [
  // {
  //   name: 'Latest',
  //   data: o0 as unknown as MockProverData,
  //   title: 'Latest Model',
  //   description: 'Debug used latest model',
  // },
  {
    name: 'Simple',
    data: o1 as unknown as MockProverData,
    title: 'Simple example',
    description: 'Simple example from halo2 book',
    sourceUrl: 'https://zcash.github.io/halo2/user/simple-example.html',
  },
  {
    name: 'Fib1',
    data: o2 as unknown as MockProverData,
    title: 'Fibonacci Example 1',
    description: 'Simple fibnonacci example from halo workshop',
    sourceUrl: 'https://github.com/icemelon/halo2-examples/blob/master/src/fibonacci/example1.rs',
  },
  {
    name: 'Fib2',
    data: o3 as unknown as MockProverData,
    title: 'Fibonacci Example 2',
    description: 'Simple fibnonacci example from halo workshop',
    sourceUrl: 'https://github.com/icemelon/halo2-examples/blob/master/src/fibonacci/example2.rs',
  },
  {
    name: 'Fib3',
    data: o4 as unknown as MockProverData,
    title: 'Fibonacci Example 3',
    description: 'Simple fibnonacci example from halo workshop',
    sourceUrl: 'https://github.com/icemelon/halo2-examples/blob/master/src/fibonacci/example3.rs',
  },
  {
    name: 'Fib4',
    data: o5 as unknown as MockProverData,
    title: 'Fibonacci Example 4',
    description: 'Simple fibnonacci example from halo workshop',
    sourceUrl: 'https://github.com/icemelon/halo2-examples/blob/master/src/fibonacci/example4.rs',
  },
  {
    name: 'Range1',
    data: o6 as unknown as MockProverData,
    title: 'Range Check Example 1',
    description: 'Simple range check example from halo workshop',
    sourceUrl: 'https://github.com/icemelon/halo2-examples/blob/master/src/range_check/example1.rs',
  },
  {
    name: 'Range2',
    data: o7 as unknown as MockProverData,
    title: 'Range Check Example 2',
    description: 'Simple range check example from halo workshop',
    sourceUrl: 'https://github.com/icemelon/halo2-examples/blob/master/src/range_check/example2.rs',
  },
  {
    name: 'Range3Broken',
    data: o8 as unknown as MockProverData,
    title: 'Range Check Example 3',
    description: 'Simple range check example from halo workshop',
    sourceUrl: 'https://github.com/icemelon/halo2-examples/blob/master/src/range_check/example3_broken.rs',
  },
  {
    name: 'MerklePath',
    data: o9 as unknown as MockProverData,
    title: 'Merkle Path(Poseidon)',
    description: 'Merkle path in poseidon hash in semaphore repo',
    sourceUrl: 'https://github.com/akinovak/halo2-semaphore',
  },
];

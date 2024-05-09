import { writable } from 'svelte/store';

type selectionType = {
  nodeIds: number[],
  edgeId: number | undefined,
}

const { subscribe, set, update } = writable<selectionType>({
  nodeIds: [],
  edgeId: undefined
});

const selectNode = (nodeId: number) =>
  update((selection) => {
    if (selection.nodeIds.length > 1) {
      return {
        nodeIds: [nodeId],
        edgeId: undefined
      };
    } else {
      return {
        nodeIds: selection.nodeIds[0] == nodeId ? [] : [nodeId],
        edgeId: undefined
      };
    }
  });

const selectEdge = (edgeId: number, nodeIds: number[]) =>
  update((selection) => {
    if (selection.edgeId == edgeId) {
      return {
        nodeIds: [],
        edgeId: undefined
      };
    } else {
      return {
        nodeIds: nodeIds,
        edgeId: edgeId
      };
    }
  });

const clear = () =>
  set({
    nodeIds: [],
    edgeId: undefined
  });

export default {
  subscribe,
  selectNode,
  selectEdge,
  clear,
};
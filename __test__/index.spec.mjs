import test from 'ava';

import { insertText } from '../index.js';

test('Invoke insertText with empty string', (t) => {
	insertText('');
	t.pass();
});

import React, {SVGProps} from 'react';

type IconProps = SVGProps<SVGSVGElement> & {
	secondaryfill?: string;
	strokewidth?: number;
	title?: string;
}

function RectLayoutGrid({fill = 'currentColor', secondaryfill, title = 'badge 13', ...props}: IconProps) {
	secondaryfill = secondaryfill || fill;

	return (
		<svg height="24" style={{}} width="24" {...props} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
	<title>{title}</title>
	<g fill="none">
		<path d="M7 21H5C2.79086 21 1 19.2091 1 17V8C1 7.44772 1.44772 7 2 7H7V21ZM22 7C22.5523 7 23 7.44772 23 8V17C23 19.2091 21.2091 21 19 21H9V7H22Z" fill="url(#1752500502800-5968645_rect-layout-grid_existing_0_ac7f60sk2)" mask="url(#1752500502800-5968645_rect-layout-grid_mask_y7d1zjmfi)" data-glass="origin"/>
		<path clipPath="url(#1752500502800-5968645_rect-layout-grid_clipPath_dzpsynprw)" d="M7 21H5C2.79086 21 1 19.2091 1 17V8C1 7.44772 1.44772 7 2 7H7V21ZM22 7C22.5523 7 23 7.44772 23 8V17C23 19.2091 21.2091 21 19 21H9V7H22Z" fill="url(#1752500502800-5968645_rect-layout-grid_existing_0_ac7f60sk2)" data-glass="clone"/>
		<path d="M1 9C1 7.13623 1 6.20435 1.30448 5.46927C1.71046 4.48915 2.48915 3.71046 3.46927 3.30448C4.20435 3 5.13623 3 7 3L17 3C18.8638 3 19.7956 3 20.5307 3.30448C21.5108 3.71046 22.2895 4.48915 22.6955 5.46927C23 6.20435 23 7.13623 23 9L1 9Z" fill="url(#1752500502800-5968645_rect-layout-grid_existing_1_jtzvnonoi)" data-glass="blur"/>
		<path d="M17 3C18.8635 3 19.7952 3.00034 20.5303 3.30469C21.5104 3.71066 22.2893 4.48961 22.6953 5.46973C22.9997 6.20476 23 7.1365 23 9L1 9C1 7.25255 1.00002 6.3243 1.25098 5.60938L1.30469 5.46973C1.68532 4.5508 2.39336 3.8084 3.28809 3.38477L3.46973 3.30469C4.20476 3.00034 5.1365 3 7 3L17 3ZM7 3.75C6.05786 3.75 5.38868 3.75022 4.8623 3.78613C4.34273 3.82159 4.01676 3.889 3.75586 3.99707C2.95971 4.32692 2.32692 4.95971 1.99707 5.75586C1.889 6.01676 1.82159 6.34273 1.78613 6.8623C1.76057 7.2369 1.75517 7.68389 1.75293 8.25L22.2471 8.25C22.2448 7.68389 22.2394 7.2369 22.2139 6.8623C22.1784 6.34273 22.111 6.01676 22.0029 5.75586C21.6731 4.95971 21.0403 4.32692 20.2441 3.99707C19.9832 3.889 19.6573 3.82159 19.1377 3.78613C18.6113 3.75022 17.9421 3.75 17 3.75L7 3.75Z" fill="url(#1752500502800-5968645_rect-layout-grid_existing_2_z4z6i1dh1)"/>
		<defs>
			<linearGradient id="1752500502800-5968645_rect-layout-grid_existing_0_ac7f60sk2" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1="7" y2="21">
				<stop stopColor="#575757"/>
				<stop offset="1" stopColor="#151515"/>
			</linearGradient>
			<linearGradient id="1752500502800-5968645_rect-layout-grid_existing_1_jtzvnonoi" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1="3" y2="9">
				<stop stopColor="#E3E3E5" stopOpacity=".6"/>
				<stop offset="1" stopColor="#BBBBC0" stopOpacity=".6"/>
			</linearGradient>
			<linearGradient id="1752500502800-5968645_rect-layout-grid_existing_2_z4z6i1dh1" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1="3" y2="6.475">
				<stop stopColor="#fff"/>
				<stop offset="1" stopColor="#fff" stopOpacity="0"/>
			</linearGradient>
			<clipPath id="1752500502800-5968645_rect-layout-grid_clipPath_dzpsynprw">
				<path d="M1 9C1 7.13623 1 6.20435 1.30448 5.46927C1.71046 4.48915 2.48915 3.71046 3.46927 3.30448C4.20435 3 5.13623 3 7 3L17 3C18.8638 3 19.7956 3 20.5307 3.30448C21.5108 3.71046 22.2895 4.48915 22.6955 5.46927C23 6.20435 23 7.13623 23 9L1 9Z" fill="url(#1752500502800-5968645_rect-layout-grid_existing_1_jtzvnonoi)"/>
			</clipPath>
			<mask id="1752500502800-5968645_rect-layout-grid_mask_y7d1zjmfi">
				<rect height="100%" width="100%" fill="#FFF"/>
				<path d="M1 9C1 7.13623 1 6.20435 1.30448 5.46927C1.71046 4.48915 2.48915 3.71046 3.46927 3.30448C4.20435 3 5.13623 3 7 3L17 3C18.8638 3 19.7956 3 20.5307 3.30448C21.5108 3.71046 22.2895 4.48915 22.6955 5.46927C23 6.20435 23 7.13623 23 9L1 9Z" fill="#000"/>
			</mask>
		</defs>
	</g>
</svg>
	);
};

export default RectLayoutGrid;